// 不能使用 crate::来引用(报函数 not in the root)，因为main.rs和lib.rs是不同的crate，虽然都是根节点
use rust::eat_at_restaurant_within_lib_rs; // 当放在lib.rs中使用

mod ch7_front_of_house;
// rust会找与此模块同名的文件中加载内容
use crate::ch7_front_of_house::hosting;

mod ch10_trait;
use crate::ch10_trait::t10_trait;

mod ch11_testing;
mod ch13_closure;
use crate::ch13_closure::t13_closure;

mod ch15_smart_pointer;
use crate::ch15_smart_pointer::t15_smart_pointer;

fn main() {
    println!("Hello, world!");
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);
    println!("{:?}", none);

    eat_at_restaurant_within_lib_rs(); // 当定义在lib.rs中使用
    hosting::add_to_waitlist();

    t10_trait();
    t13_closure();
    t15_smart_pointer();
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
