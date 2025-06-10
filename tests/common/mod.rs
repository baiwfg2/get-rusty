// 为公共函数创建mod.rs，不是 tests/common.rs，后者会造成被看成是带有用例的包，进入到了测试输出中
pub fn setup() {
    println!("basic setup in tests/common");
}