/* 不能使用 crate::来引用(报函数 not in the root)，
因为main.rs和lib.rs是不同的crate，虽然都是根节点, crate::
只能引用当前 crate 内的内容

rust:: = 外部 crate 的名字
rust 是你的 package 名字（来自 Cargo.toml 的 name = "rust"）
*/
use get_rusty::eat_at_restaurant_within_lib_rs; // 当放在lib.rs中使用

mod ch1_4;
use crate::ch1_4::t_ch1_ch4;

mod ch7_front_of_house;
// rust会找与此模块同名的文件中加载内容
use crate::ch7_front_of_house::hosting;

mod ch10_trait;
use crate::ch10_trait::t10_trait;
mod ch10_lifetime;
use crate::ch10_lifetime::t10_lifetime;

mod ch11_testing;
mod ch12_io_tools;
use ch12_io_tools::t12_main;

mod ch13_closure;
use crate::ch13_closure::t13_closure;

mod ch15_smart_pointer;
use crate::ch15_smart_pointer::t15_smart_pointer;

mod ch16_concurrency;
use crate::ch16_concurrency::t16_concurrency;

mod ch17_oop;
use crate::ch17_oop::t17_oop;

mod ch17_async;
use crate::ch17_async::t17_async;

mod ch18_pattern_match;
use crate::ch18_pattern_match::t18_pattern_match;

mod ch19_advanced_feature;
use crate::ch19_advanced_feature::t19_advanced_feature;

mod external;
use external::tokio_ex::t_tokio;
use external::pin_ex::t_pin;

mod async_programming;

use arrayvec::ArrayVec;

// ArrayVec 是 Rust 中一个基于数组的、具有固定容量的向量类型。它允许将数据内联存储，这意味着数据可以
// 存储在栈上，避免了堆分配，从而提高了性能
fn t_arrayvec() {
    // Create an ArrayVec with a capacity of 16
    let mut array_vec = ArrayVec::<_, 16>::new();

    // Push elements into the ArrayVec
    array_vec.push(1);
    array_vec.push(2);
    array_vec.push(3);

    // Access elements like a slice
    println!("Elements: {:?}", &array_vec[..]); // Output: Elements: [1, 2, 3]

    // Check capacity and length
    println!("Capacity: {}", array_vec.capacity()); // Output: Capacity: 16
    println!("Length: {}", array_vec.len());     // Output: Length: 3

    // Attempting to push beyond capacity will result in an error or panic
    // depending on the method used (e.g., `try_push` returns a Result)
}

use std::borrow::Cow;

fn process_string(input: &str) -> Cow<'_, str> {
    if input.contains("bad_word") {
        // If the string needs modification, return an owned String
        Cow::Owned(input.replace("bad_word", "good_word"))
    } else {
        // Otherwise, return a borrowed slice
        Cow::Borrowed(input)
    }
}

/*
Cow<str> 在 Rust 中是一个智能指针，它实现了“写时复制”的功能。它允许你既可以拥有借用的数据，也可以在需要时进行复制。这种设计常用于字符串，当字符串可能是 &str（借用的字符串切片） 或 String（拥有所有权的字符串）时，可以避免不必要的内存分配。如果 Cow<str> 包装的是 &str，那么它不会进行复制，直到你需要修改数据时才会进行克隆，从而实现延迟复制，提高效率
*/
fn t_cow_str() {
    let s1 = "This is a clean string.";
    let result1 = process_string(s1);
    println!("Result 1: {}", result1); // Prints "This is a clean string."

    let s2 = "This contains a bad_word.";
    let result2 = process_string(s2);
    println!("Result 2: {}", result2); // Prints "This contains a good_word."
}

fn main() {
    println!("Hello, world!");
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);
    println!("{:?}", none);

    //eat_at_restaurant_within_lib_rs(); // 当定义在lib.rs中使用
    //hosting::add_to_waitlist();

    //t_ch1_ch4();
    //t10_trait();
    //t10_lifetime();
    //t12_main();
    //t13_closure();
    //t15_smart_pointer();
    //t16_concurrency();
    //t17_oop();
    //t17_async();
    //t18_pattern_match();
    //t19_advanced_feature();
    //ch20_web::t20_webserver_main();

    //t_arrayvec();
    //t_cow_str();
    //t_tokio();
    //t_pin();

    async_programming::async_prog::t_async_main();
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
