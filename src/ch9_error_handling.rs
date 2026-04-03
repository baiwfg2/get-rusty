use std::fs::File;
use std::io::ErrorKind;

fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    // 下面的代码也可以用 ? 来简化
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?; // 只能与 Result<T, E> 类型的值一起使用
    // Ok(s)
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn t9_error_handling() {
    let f = File::open("hello.txt");
    // manual match
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let f2 = File::open("hello.txt").expect("Failed to open hello.txt");
}