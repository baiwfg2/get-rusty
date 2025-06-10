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

// similar to template specilization
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

// this is a short form of Trait as parameters, suitable for simple occasion
// if `impl` is omitted, then error reminds: 
//      alternatively, use a trait object to accept any type that implements `Summary`(That is impl Summary), 
//      accessing its methods at runtime using dynamic dispatch
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// the above is equivalent to this form
fn notifyWithTraitConstraint<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

fn notifyMakeParameterUseSameType<T: Summary>(item1: T, item2: T) {}


// fn someFunctionUsingWhereClause<T,U>(t: T, u: U) -> i32 
//     where T: Display + Clont,
//           U: Clone + Debug {}

///////// return Trait type
// cannot return differnt type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

/*
If not annotated with lifetime tag, error will be: expected named lifetime parameter
Because compiler's borrow checker cannot figure out which &str will be returned
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerptWithLifetimeTag<'a> {
    // annoatate every ref when there're refs in struct
    part: &'a str,
}

impl<'a> ImportantExcerptWithLifetimeTag<'a> {
    // no ref parameter, no ref return
    fn level(&self) -> i32 {
        3
    }

    // Rule 1: apply annotate to self('a) and ann('b)
    // Rule 3: appy self's lifetime to returnValue
    // so that every refs can be annotated without human annotation
    fn announce_and_return_part(&self, ann: &str) -> &str {
        println!("Attention please:{}", ann);
        self.part
    }
}

use std::fmt::Display;
// use generics, trait constrait, lifetime simutaneously
fn useGenericsAndTraitconstraintAndLifetime<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
    where T: Display {
    println!("[useGenericsAndTraitconstraintAndLifetime] announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn t_lifetime() {
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let res = longest(s1.as_str(), s2);
    println!("[LIFETIME exercise] the longest string is {}", res);

    ///////// lifetime anti-example
    let s3 = String::from("abcd");
    let res2;
    {
        let s4 = String::from("xyz");
        res2 = longest(s3.as_str(), s4.as_str());
    }
    // error: borrowed value does not live long enough if calling this. No error if not called
    // println!("the longest string is {}", res2);

    let novel = String::from("call me cshi. some years aog....");
    let first_sentence = novel.split('.').next().expect("ERROR !!");
    let i = ImportantExcerptWithLifetimeTag { part: first_sentence };
    useGenericsAndTraitconstraintAndLifetime(s1.as_str(), s2, 1);
}

pub fn t10() {
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

    t_lifetime();
}
