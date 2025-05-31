// lib.rs is added to build automatically
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to waitlist");
        }
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
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
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
    }
}