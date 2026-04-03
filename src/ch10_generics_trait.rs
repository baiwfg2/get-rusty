// 包进mod后，外部调用时，得保证scope 是pub
mod generics {

// 这种写法不利于消除代码冗余
pub fn largestWithExplicitType(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//////////// 以下largest 版本是我自己琢磨出来的，需仔细体会 (P287 有类似的版本，但还没有我的丰富)

// 如果没有PartialOrd，报 binary operation `>` cannot be applied to type `T`
// 如果没Copy, 报：cannot move out of type `[T]`, a non-copy slice
//      move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
// 即使是 &mut [T] 也不能直接把元素 move 出来（和 &[T] 一样），因为切片只是借用，不能把借来的位置留空
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/*  在第一版基础，编译器提示可以返回引用，这样就不用要求T实现Copy trait了

15 | fn largest<T: PartialOrd>(list: &[T]) -> T {
   |            ^ consider constraining this type parameter with `Clone`
16 |     let mut largest = list[0];
   |                       ------- you could clone this value
help: consider borrowing here
   |
16 |     let mut largest = &list[0];
   |                       +
error[E0507]: cannot move out of a shared reference
  --> src/ch10_generics_trait.rs:17:18
   |
17 |     for &item in list.iter() {
   |          ----    ^^^^^^^^^^^
   |          |
   |          data moved here
   |          move occurs because `item` has type `T`, which does not implement the `Copy` trait
   |
help: consider removing the borrow
   |
17 -     for &item in list.iter() {
17 +     for item in list.iter() {
*/
fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        /* 为什么 item > largest （直接比较引用）能工作？
        这是因为 Rust 标准库针对引用提供了一揽子的 trait 实现（Blanket Implementations）。
        标准库中有一条规则：如果类型 T 实现了 PartialOrd，那么它的引用类型 &T 也自动实现了 PartialOrd。
        在比较两个引用时（&T > &T），标准库的代码会自动帮你解引用

        写成 *item > *largest */
        if item > largest {
            // 关键点：是在改变引用本身指向的位置，而不是在修改数组里的元素。
            // 它只是让 largest 这个“指针”不再指向上一个元素
            largest = item;
            // report: largest` is a `&` reference, so the data it refers to cannot be written
            //*largest = *item;
        }
    }
    largest
}

fn largest3<T: PartialOrd>(list: Vec<T>) -> T {
    /* 不能直接写成
    let mut largest = list[0];
    for item in list.into_iter() {
    因为let mut largest = list[0] 仍会要求T 实现Copy，而 into_inter内部会
    直接转移所有权，也就不要求T 实现Copy了
     */
    let mut iter = list.into_iter();
    let mut largest = iter.next().expect("list cannot be empty");
    for item in iter {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 与largest3 有区别！！
// (&Vec<T>).into_iter() 生成 &T（生成引用 / Borrow），在语义上完全等同于直接调用 .iter()
fn largest4<T: PartialOrd>(list: &Vec<T>) -> &T {
    let mut iter = list.into_iter();
    let mut largest = iter.next().expect("list cannot be empty");
    for item in iter {
        // 这里和 largest2 一样，自动解引用
        if item > largest {
            largest = item;
        }
    }
    largest
}

/*
the good news is that using generic types won't make your program run any slower than
it would iwth concrete types.
Rust accomplishes this by performing monomorphization of the code using generics at compile
time. Monomorphization is the process of tunring generic code into specific code by
filling in the concrete types that are used when compiled.
 */
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

/*
我们必须紧跟着impl关键字声明T，以便能够在实现方法时指定类型Point<T>。通过在impl之后将T声明为泛型，
Rust能够识别出Point尖括号内的类型是泛型而不是具体类型
打个比方， 我们可以单独为Point<f32> 实例而不是所有的Point<T>泛型实例来实现方法
 */
impl<T> Point<T> {
    pub fn x(&self) -> &T {
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

/*
它和 C++ 模板一样，使用的是编译期多态（静态派发，Static Dispatch）。
在 Rust 中，这个过程叫作单态化（Monomorphization）

C++ 传统的模板是“鸭子类型（Duck Typing）”，传什么进去都在编译展开时才报错，报错信息经常是一座长篇大论的大山。
Rust 的 impl Trait / Trait Bound 更像是 C++20 引入的 Concepts。你在声明参数时就加上了契约（Summary），
如果在调用处传入未实现该 trait 的类型，Rust 在“调用点”就会立刻给你精准而简明扼要的报错，而不需要等到内部展开。
*/
mod use_trait_as_parameter {
    use std::fmt::{Debug, Display};
    use super::Summary;

    // this is a short form of Trait as parameters（是trait bound的语法糖）, suitable for simple occasion
    // if `impl` is omitted, then error reminds:
    //      alternatively, use a trait object to accept any type that implements `Summary`(That is impl Summary),
    //      accessing its methods at runtime using dynamic dispatch
    // pub fn notify(item: &dyn Summary) {

    // you can also use an opaque type, but users won't be able to specify the type parameter when calling
    // the `fn`, having to rely exclusively on type inference
    // 注意：没有写成 &impl Summary ，则可能会move item
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

    /* Rust 是一门静态类型语言，在编译期必须确切知道每个函数的返回值到底占用多少字节的
    内存。如果你在 if 分支里返回 NewsArticle，在 else 分支里返回 Tweet，这两个结构
    体的内存大小和布局是不同的，编译器无法为这个函数确定一个统一的返回类型，因此直接报错 */
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

// mod generics; 意思是“去寻找名为generics.rs 或 generics/mod.rs 的外部文
// 件并声明为模块”，这会导致重复声明或者找不到文件的错误
use generics::*;

pub fn t10_trait() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largestWithExplicitType(&number_list);
    println!("The largest number using normal function is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let res2= largest(&char_list);
    println!("The largest char using generic type function is {}", res2);
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
