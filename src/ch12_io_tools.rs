use std::env;
use std::fs;
use std::process;

use std::error::Error;

use get_rusty::{search, search_case_insensitive};

// RUSTFLAGS=-Awarnings IGNORE_CASE=1 cargo run --target-dir /data2/rust-target/ -- CHECK a.md
pub fn t12_main() {
    let args: Vec<String> = env::args().collect();
    /*
    Invoking the macro on an expression moves and takes ownership of it before returning the evaluated
    expression unchanged. If the type of the expression does not implement Copy and you don't
    want to give up ownership,you can instead borrow with dbg!(&expr) for some expression expr.

    The dbg! macro works exactly the same in release builds. This is useful when debugging issues that
    only occur in release builds or when debugging in release mode is significantly faster.
     */
    dbg!(&args);

    // 重构：将解析配置参数的代码移到单独的函数中
    // let query = &args[1];
    // let filename = &args[2];

    // let config = parse_config(&args);
    //let config = Config::new(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguemnts: {err}");
        process::exit(1);
    });
    // 不关心run 正确时的返回值，只关心错误时的处理
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// () 指 unit type
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 这种expect 方式不好，出现任何报错都变成了这一句话
    // let contents = fs::read_to_string(config.filename)
    //     .expect("Something went wrong reading the file");

    // ? 负责发生错误时自动转换：Err(e.into()) 并返回，contents不会被赋值
    // 如果没有 ?  contents 的类型是 Result<String, std::io::Error>
    let contents = fs::read_to_string(config.filename)?; // ? return error value from the current function for the caller to handle

    let res = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in res {
        println!("{line}");
    }
    return Ok(());
}

struct Config {
    query: String,
    filename: String,
    pub ignore_case: bool,
}

impl Config {
    // 许多人会假定new 不会fail，因此当换成返回Result时，换一种函数名称
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            // 对于usage problem，不应该使用panic，更好的方式是返回Result类型
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        // 这里不关注是什么值，只关注是否存在
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Config { query, filename, ignore_case }
    }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
         // 这里不关注是什么值，只关注是否存在
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, filename, ignore_case })
    }
}

// 使用 Config::new 的方式更idiomatic
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    //let query = args[1]; //  move occurs because `args[_]` has type `std::string::String`, which does not implement the `Copy` trait
    let filename = args[2].clone();
    Config { query, filename, ignore_case: false }
}