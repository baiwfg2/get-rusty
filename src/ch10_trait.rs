pub fn largestByNormalWay(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 会报错 T没有实现 PartialOrd trait
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// similar to template specilization (P273)
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum Option<T, E> {
    Some(T),
    None,
    Ok(T),
    Err(E),
}

// ch10 trait
pub trait Summary {
    fn summarize(&self) -> String {
        //String a = realVirtualFunc(); // default func can call non-default, not vice versa
        // default implementation
        String::from("read more ...")
    }

    //fn realVirtualFunc(&self) -> String; // must be implemented
}

////////// do different things under same function for different types
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // when using defalult implementation, leave body blank
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

mod use_trait_as_parameter {
    use std::fmt::{Debug, Display};
    use super::Summary;

    // this is a short form of Trait as parameters, suitable for simple occasion
    // if `impl` is omitted, then error reminds:
    //      alternatively, use a trait object to accept any type that implements `Summary`(That is impl Summary),
    //      accessing its methods at runtime using dynamic dispatch
    pub fn notify(item: impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // This form is called a `trait bound` (Eng ver P243), equivalent to above form (P283)
    fn notifyWithTraitConstraint<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }

    // 如果要求两个参数是同一种类型，只能这样写，不能写成 impl Summary
    fn notifyMakeParameterUseSameType<T: Summary>(item1: T, item2: T) {}


    fn someFunctionUsingWhereClause<T,U>(t: T, u: U)
        where T: Display + Clone,
            U: Clone + Debug {}
}

///////// return Trait type
// cannot return differnt type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }

    /* NO support this
    if switch {
        NewsArticle
    } else {
        Tweet
    }
     */
}

mod implement_trait_function_condionally {
    use std::fmt::Display;

    // 泛型结构体
    struct Pair<T> {
        x: T,
        y: T,
    }

    // 为所有T实现Pair<T>的方法
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // 仅为实现了Display和PartialOrd trait的T实现cmp_display方法
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

pub fn t10_trait() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largestByNormalWay(&number_list);
    println!("The largest number using normal function is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let res2= largest(&char_list);
    println!("The largest char using trait-style function is {}", res2);
    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 1.0, y:2.0 };
    println!("integer.x = {}", integer.x());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    // 因为整形实现了 ToString trait
    println!("blanket impl: {}", 3.to_string());
}
