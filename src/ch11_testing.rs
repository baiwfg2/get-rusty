pub fn greeting(name: &str) -> String {
    format!("hello {}!", name)
}

pub fn greetingBad(name: &str) -> String {
    format!("hello")
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("wrong value:{}", value);
        }
        Guess { value }
    }
}

#[cfg(test)] // 单元测试和源文件放在一起，用此标注可在cargo build时避免编译test 产物
mod tests {
    use crate::ch11_testing;

    use super::*;

    #[test]
    fn test_greeting() {
        let res = greeting("Carol");
        assert!(res.contains("Carol"));
    }

    #[test]
    fn test_greetingBad() {
        let res = greetingBad("Carol");
        // add customized reason-explaining statement
        //assert!(res.contains("Carol"), ">> Greeting did not contain name, value was {}", res);
    }

    #[test]
    #[should_panic(expected="wrong value:")]
    fn test_GuessNew() {
        Guess::new(200);
    }

    #[test]
    // use Result<T, E> to return error instead of panic
    fn it_works() -> Result<(), String> {
        if 2+2 == 4 {
            Ok(())
        } else {
            Err(String::from("2+2 != 4"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {}
}

/*
common arguments:
--test-threads
cargo test -- --nocapture
cargo test xxxTest: run only filered cases (测试所在的模块的名称也是测试名称的一部分)
cargo test -- --ignored
cargo tet --test integration_test_filename
*/