//use add_one;

fn main() {
    let num = 10;
    // 如果故意写成 add_one2，则报：use of unresolved module or unlinked crate `add_one2
    // 如果移除Cargo.toml中的add-one依赖，同样也会报这个
    println!("Hello, world. {} plus one is {}", num, add_one::add_one(num));
}

/*
cargo build
cargo run -p adder
*/