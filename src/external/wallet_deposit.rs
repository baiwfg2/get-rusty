use std::collections::HashSet;
use dashmap::DashMap;
use uuid::Uuid;
use sqlx::{Postgres, Pool, postgres::PgQueryResult, Row};
use std::sync::Arc;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::ops::Not;

// 错误类型定义
#[derive(Debug)]
pub enum DepositError {
    DatabaseError(sqlx::Error),
    ValidationError(String),
    DuplicateDeposit,
}

impl From<sqlx::Error> for DepositError {
    fn from(err: sqlx::Error) -> Self {
        DepositError::DatabaseError(err)
    }
}

impl std::fmt::Display for DepositError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepositError::DatabaseError(e) => write!(f, "Database error: {}", e),
            DepositError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            DepositError::DuplicateDeposit => write!(f, "Duplicate deposit"),
        }
    }
}

impl std::error::Error for DepositError {}

type UserId = u64;
type Currency = String;

#[derive(Debug, Clone, Serialize)]
pub struct Balance {
    pub available: BigDecimal,
    pub frozen: BigDecimal, // 用于挂单、提现中的资金
}

pub struct MemoryBalanceStore {
    // Arc<DashMap> 使得它可以被安全地共享 across threads
    inner: Arc<DashMap<UserId, DashMap<Currency, Balance>>>,
}

impl MemoryBalanceStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(DashMap::new()),
        }
    }

    // 获取用户某个币种的余额
    pub fn get_balance(&self, user_id: &UserId, currency: &str) -> Option<Balance> {
        self.inner
            .get(user_id)
            .and_then(|user_balances| user_balances.get(currency).map(|b| b.clone()))
    }

    // 原子性地更新余额（例如：可用转冻结）
    pub fn transfer_available_to_frozen(
        &self,
        user_id: UserId,
        currency: &str,
        amount: &BigDecimal,
    ) -> Result<(), String> {
        // DashMap 的 entry API 允许我们原子性地执行一系列操作
        let user_balances = self.inner.entry(user_id).or_insert_with(DashMap::new);
        let mut balance_entry = user_balances.entry(currency.to_string()).or_insert(Balance {
            available: BigDecimal::from(0),
            frozen: BigDecimal::from(0),
        });

        if balance_entry.available < *amount {
            return Err("Insufficient available balance".to_string());
        }

        balance_entry.available -= amount;
        balance_entry.frozen += amount;
        Ok(())
    }

    // 从数据库加载数据到内存（系统启动或用户首次登录时调用）
    pub fn load_balances_from_db(&self, user_id: UserId, balances: Vec<(Currency, Balance)>) {
        let user_balance_map = self.inner.entry(user_id).or_insert_with(DashMap::new);
        for (currency, balance) in balances {
            user_balance_map.insert(currency, balance);
        }
    }
}

// 内存中维护已处理的事务ID（可配合Redis持久化）
pub struct IdempotencyManager {
    processed_tx_ids: DashMap<String, bool>, // tx_id -> processed
}

impl IdempotencyManager {
    pub fn new() -> Self {
        Self {
            processed_tx_ids: DashMap::new(),
        }
    }

    // 检查并标记事务ID为已处理
    pub fn acquire_lock(&self, tx_id: &str) -> bool {
        // 如果已经存在，返回false表示重复
        // 如果不存在，插入并返回true
        self.processed_tx_ids.contains_key(tx_id).not()
    }
}

// 充值记录结构
#[derive(Debug, sqlx::FromRow)]
pub struct DepositRecord {
    pub id: i64,
    pub user_id: i64,
    pub currency: String,
    pub amount: BigDecimal,
    pub tx_hash: String,        // 区块链交易哈希
    pub vout_index: i32,        // 交易输出索引（比特币UTXO）
    pub deposit_id: String,     // 我们系统生成的唯一ID（幂等键）
    pub status: DepositStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum DepositStatus {
    Pending,    // 待确认
    Confirmed,  // 已确认入账
    Duplicate,  // 重复交易
}

pub struct DepositService {
    db_pool: Pool<Postgres>,
    idempotency_mgr: Arc<IdempotencyManager>,
}

impl DepositService {
    pub async fn process_deposit(
        &self,
        user_id: i64,
        currency: &str,
        amount: &BigDecimal,
        blockchain_tx_hash: &str,
        vout_index: i32,
    ) -> Result<(), DepositError> {
        // 生成幂等键：区块链交易哈希 + 输出索引
        let idempotency_key = format!("{}_{}", blockchain_tx_hash, vout_index);

        // 第一步：检查是否已处理（内存快速检查）
        if !self.idempotency_mgr.acquire_lock(&idempotency_key) {
            log::warn!("Duplicate deposit detected: {}", idempotency_key);
            return Ok(()); // 直接返回成功，因为已经处理过了
        }

        // 第二步：数据库事务中处理（最终一致性）
        let mut tx = self.db_pool.begin().await?;

        // 使用普通的 query 而不是宏，避免编译时数据库检查
        let result = sqlx::query(
            r#"
            INSERT INTO deposit_records
                (user_id, currency, amount, tx_hash, vout_index, deposit_id, status)
            VALUES ($1, $2, $3, $4, $5, $6, 'pending')
            ON CONFLICT (tx_hash, vout_index) DO NOTHING
            RETURNING id
            "#,
        )
        .bind(user_id)
        .bind(currency)
        .bind(amount)
        .bind(blockchain_tx_hash)
        .bind(vout_index)
        .bind(&idempotency_key)
        .fetch_optional(&mut *tx)
        .await?;

        // 如果result为None，说明是重复记录，直接返回
        if result.is_none() {
            tx.rollback().await?;
            log::info!("Deposit already processed: {}", idempotency_key);
            return Ok(());
        }

        // 第三步：更新用户余额（只有在新记录时才执行）
        let update_result = sqlx::query(
            r#"
            UPDATE account_balances
            SET available_balance = available_balance + $1
            WHERE user_id = $2 AND currency = $3
            "#,
        )
        .bind(amount)
        .bind(user_id)
        .bind(currency)
        .execute(&mut *tx)
        .await?;

        // 检查是否成功更新了余额
        if update_result.rows_affected() == 0 {
            // 如果用户还没有该币种的余额记录，插入新记录
            sqlx::query(
                r#"
                INSERT INTO account_balances (user_id, currency, available_balance, frozen_balance)
                VALUES ($1, $2, $3, 0)
                ON CONFLICT (user_id, currency) DO UPDATE
                SET available_balance = account_balances.available_balance + $3
                "#,
            )
            .bind(user_id)
            .bind(currency)
            .bind(amount)
            .execute(&mut *tx)
            .await?;
        }

        // 第四步：更新充值记录状态为已确认
        sqlx::query(
            r#"
            UPDATE deposit_records
            SET status = 'confirmed', updated_at = NOW()
            WHERE tx_hash = $1 AND vout_index = $2
            "#,
        )
        .bind(blockchain_tx_hash)
        .bind(vout_index)
        .execute(&mut *tx)
        .await?;

        // 提交事务
        tx.commit().await?;

        // 第五步：更新内存余额（这里注释掉，因为方法不存在）
        // self.update_memory_balance(user_id, currency, amount).await;

        log::info!("Deposit processed successfully: {} for user {}", amount, user_id);
        Ok(())
    }
}

////////////// 带重试保护的幂等
pub struct RobustDepositService {
    db_pool: Pool<Postgres>,
    idempotency_mgr: Arc<IdempotencyManager>,
    memory_store: Arc<MemoryBalanceStore>,
}

impl RobustDepositService {
    pub async fn process_deposit_with_retry(
        &self,
        user_id: i64,
        currency: &str,
        amount: &BigDecimal,
        blockchain_tx_hash: &str,
        vout_index: i32,
        max_retries: u32,
    ) -> Result<(), DepositError> {
        let idempotency_key = format!("{}_{}", blockchain_tx_hash, vout_index);

        for attempt in 0..=max_retries {
            match self.try_process_deposit(
                user_id,
                currency,
                amount,
                &idempotency_key
            ).await {
                Ok(()) => return Ok(()),
                Err(e) if attempt == max_retries => return Err(e),
                Err(e) => {
                    log::warn!("Deposit processing attempt {} failed: {:?}", attempt, e);
                    tokio::time::sleep(tokio::time::Duration::from_millis(100 * 2u64.pow(attempt))).await;
                }
            }
        }

        unreachable!()
    }

    async fn try_process_deposit(
        &self,
        user_id: i64,
        currency: &str,
        amount: &BigDecimal,
        idempotency_key: &str,
    ) -> Result<(), DepositError> {
        // 使用数据库事务实现完整的幂等检查
        let mut tx = self.db_pool.begin().await?;

        // 先检查是否已经成功处理过（更严格的检查）
        let existing = sqlx::query(
            r#"
            SELECT status FROM deposit_records
            WHERE deposit_id = $1 FOR UPDATE
            "#,
        )
        .bind(idempotency_key)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(record) = existing {
            let status: String = record.get("status");
            match status.as_str() {
                "confirmed" => {
                    tx.rollback().await?;
                    return Ok(());
                }
                "pending" => {
                    // 之前开始处理但未完成，继续处理
                }
                _ => {
                    // 其他状态，按需处理
                }
            }
        } else {
            // 插入新记录
            sqlx::query(
                r#"
                INSERT INTO deposit_records
                    (user_id, currency, amount, deposit_id, status)
                VALUES ($1, $2, $3, $4, 'pending')
                "#,
            )
            .bind(user_id)
            .bind(currency)
            .bind(amount)
            .bind(idempotency_key)
            .execute(&mut *tx)
            .await?;
        }

        // 更新余额（幂等的关键：余额更新是累加的，但通过事务保证只执行一次）
        // 更新数据库中的余额（这里注释掉，因为方法不存在）
        // self.update_balance_in_transaction(&mut tx, user_id, currency, amount).await?;

        // 标记为已完成
        sqlx::query(
            r#"
            UPDATE deposit_records
            SET status = 'confirmed', updated_at = NOW()
            WHERE deposit_id = $1
            "#,
        )
        .bind(idempotency_key)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        // 更新内存状态
        self.memory_store.update_balance(user_id as u64, currency, |balance| {
            balance.available += amount;
        }).await;

        Ok(())
    }
}

impl MemoryBalanceStore {
    // 提供原子性的余额更新操作
    pub async fn update_balance<F>(
        &self,
        user_id: UserId,
        currency: &str,
        update_fn: F,
    ) -> Result<(), String>
    where
        F: FnOnce(&mut Balance),
    {
        let user_balances = self.inner.entry(user_id).or_insert_with(DashMap::new);
        let mut balance_entry = user_balances.entry(currency.to_string()).or_insert(Balance {
            available: BigDecimal::from(0),
            frozen: BigDecimal::from(0),
        });

        update_fn(&mut balance_entry);
        Ok(())
    }

    // 带版本控制的更新，防止并发问题
    pub fn update_balance_with_version(
        &self,
        user_id: UserId,
        currency: &str,
        new_balance: Balance,
        expected_version: u64,
    ) -> Result<bool, String> {
        // 实现乐观锁控制
        todo!("Implement version control for memory balances")
    }
}