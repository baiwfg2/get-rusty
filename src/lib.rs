//! # My LIB CRATE
//! 
//! contains part of code in trpl

// lib.rs is added to build automatically
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to waitlist within lib.rs");
        }
    }
}

// use crate::front_of_house::hosting; // absolute path
    // pub use crate::front_of_house::hosting 这叫re-importing，可以使得外部代码也可直接使用。但我觉得这不好，外部代码想用时，应该直接使用crate::front_of_house::hosting
use self::front_of_house::hosting; // relative path, 不能和crate::front_of_house::hosting同时使用，否则报 reimported error
    // 不使用 self:: 似乎也可以
use std::fmt::Result;
use std::io::Result as IoResult; // 重命名避免冲突
use std::{cmp::Ordering, io};
// use std::io::{self, Write};
// use std::collections::*;

pub fn eat_at_restaurant_within_lib_rs() {
    println!("[ACCESSING PUBLIC MODULE within lib.rs]");
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist(); // 使用use关键字后，可以直接使用

    let mut meal = back_of_house::Breakfast::new("Rye");
    meal.toast = String::from("Wheat");
    println!("[ACCESSING PUBLIC FIELD within lib.rs] I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries");// not public field

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {
    println!("Serving order");
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super 关键字在模块树中向上移动，从而允许我们调用位于
        // 直接父级的作用域中的函数
        super::serve_order();
    }

    fn cook_order() {
        println!("Cooking order");
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, 
    }

    impl Breakfast {
        pub fn new(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 枚举的pub会使所有成员都变成pub
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

/// 将传入的两参数相加
/// 
/// # Examples
/// ```
/// use rust::add_two;
/// let ans = add_two(1, 2); // 如果有语法错误，会报出来的
/// assert_eq!(3, ans);
/// ```
pub fn add_two(a: i32, b: i32) -> i32 {
    a + b
}

// use `cargo doc --open` can see the documentation effect
