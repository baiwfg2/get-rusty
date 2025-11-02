
use std::fmt;
use std::ops::Add;
use std::rc::Rc;
use std::slice;
use std::sync::atomic::AtomicU64;
use std::sync::Mutex;
use std::vec;

// 因为 proc_macro crate 只能在 Rust 的 proc-macro 类型 crate（即宏 crate）中使用，
//   不能在普通的 binary 或 library crate 中直接 use proc_macro
// use proc_macro::TokenStream;

fn t_dereference_raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 解引用时必须用unsafe 块
    unsafe {
        println!("r1 points to: {}", *r1);
        println!("r2 points to: {}", *r2);
        *r2 += 1; // 修改原始数据
        println!("After increment, r2 points to: {}", *r2);
    }
}

// 来自标准库中的split_at_mut，返回两个切片
fn t_use_split_at_mut_from_std() {
    let mut v = vec![1, 2, 3, 4, 5];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5]);
}

// fn split_at_mut_of_mine(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     assert!(mid <= len, "mid must be less than or equal to slice length");
//     // error:  cannot borrow `*slice` as mutable more than once at a time
//     (&mut slice[..mid], &mut slice[mid..])
// }

// 对不安全的代码做了安全抽象
fn t_use_unsafe_to_implement_split_at_mut(slice: &mut [i32], mid: usize) -> 
    (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len, "mid must be less than or equal to slice length");

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

// extern 用于两种场合：一是extern crate， 二是用于FFI
unsafe extern "C" {
    fn abs(intput: i32) -> i32;
}

fn t_call_c() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

static mut COUNTER: u32 = 0;

fn add_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

/*
https://doc.rust-lang.org/nightly/edition-guide/rust-2024/static-mut-references.html
这里谈到了几个建议：
1、 不使用可变全局变量
2、 使用atomic types :  static COUNTER: AtomicU64 = AtomicU64::new(0);
3、 mutex or Rwlock: static QUEUE: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
4、使用 OnceLock, LazyLock (当只作一次初始化时)
*/
static COUNTER2: AtomicU64 = AtomicU64::new(0);
static QUEUE: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn t_use_static_variable() {
    add_count(3);
    unsafe {
        // error: shared reference to mutable static
        //println!("COUNTER: {}", COUNTER);
    }
}

fn t_advanced_trait() {
    struct Counter {}
    impl Iterator for Counter {
        // 这叫关联类型，可以定义出包含某些类型的trait，而必须在实现前确定它们的具体类型是什么 (P603)
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            Option::Some(1)
        }
    }
}


////////////// 运算符重载
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    // If no-exist, report: not all trait items implemented, missing: `type Output`
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn t_operator_overload() {
    assert!(Point { x: 1, y: 2 } + Point { x: 3, y: 4 } == Point { x: 4, y: 6 });
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("* {} *", output);
        println!("{}", "*".repeat(len + 4));
    }
}
impl OutlinePrint for Point {}

// Point必须实现Display trait才能使用outline_print
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

fn t_supertrait() {
    Point { x: 1, y: 2 }.outline_print();
}

fn t_implement_external_trait_on_external_type_using_newtype_mode() {
    // 展示如何用 newtype 模式实现外部类型的外部 trait
    #[derive(Debug)]
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Wrapper 是一个元组结构体，其0号是Vec<T>
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("Hello"), String::from("World")]);
    println!("w: {}", w);
}

fn t__type_alias() {
    // 使用别名可简化字符
    /*
    Box<dyn Fn() + Send + 'static> 表示一个装箱的闭包（函数对象），
    Fn()：无参数、无返回值的闭包 trait。
    Send：可以在线程间安全传递。
    'static：闭包里捕获的所有数据都必须拥有 'static 生命周期（即整个程序期间都有效）。
     */
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("Hi"));
    fn taks_long_type(f: Thunk) {}

    type MyResult<T> = Result<T, std::io::Error>;
}

fn t__never_type() {
    // Never type 用于表示永远不会返回的类型，这样的函数也叫diverging function
    fn bar() -> ! {
        // ! 表示永远不会返回
        panic!("This function never returns!");
    }

    let guess = String::from("42");
    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // continue的返回类型是 ! 当失败时，不会给guess赋值
        };
        break;
    }
}

fn t__dynamic_type() {
    // str类型是不能确定大小的，但 &str 却可以，底层会存储数据起始位置和长度
    trait foo {}
    let s1: Box<dyn foo>;
    let s2: Rc<dyn foo>;
    let s3: &dyn foo;
    // 以上三种类型都是动态大小类型（DST），它们的大小在编译时未知

    fn generic<T>(t: T) {} // T 默认是 Sized trait
    fn generic2<T: ?Sized>(t: &T) {} // ?Sized 表示 T 可以是动态大小类型也可以是固定大小类型
    // 如果没有写 &T ,则报：doesn't have a size known at compile-time
}

fn t_advanced_type() {
    t__type_alias();
    t__never_type();
    t__dynamic_type();
}

fn t_advanced_function_closure() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    // fn 是函数指针，要与 Fn trait 区分开来，它实现了Fn, FnMut, FnOnce(P625)
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let ans = do_twice(add_one, 5);
    assert_eq!(ans, 12);

    enum Status {
        Value(u32),
        Stop,
    }
    // 使用Status::Value 构造函数来创建 Status::Value(0), Status::Value(1) 等
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    // P625: 使用闭包trait 的泛型来编写函数，可同时处理闭包和普通函数

    // return closure
    // error:  expected a type, found a trait(无法推断多大空间来存储此返回的闭包)
    // fn returns_closure() -> Fn(i32) -> i32 {
    //     |x| x + 1
    // }

    fn returns_clousure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

}

// 用于通用元编程的声明宏
#[macro_export]
macro_rules! myvec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}

/*
操作：
cargo new hello_macro --lib
cargo new hello_macro_derive --lib

copilot:
- 不要用 use rust::HelloMacro;，因为 trait 在 hello_macro crate。
- use crate::hello_macro::HelloMacro; 只适用于同 crate 下的模块，不适用于外部 crate。
- 必须保证 hello_macro 和 hello_macro_derive 都是独立 crate，并且路径正确。
*/
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancake;

fn t_macro() {
    let v = myvec![1, 2, 3];

    // #[some_attribute]
    // pub fn some_name(input: TokenStream) -> TokenStream {
    //     // 处理输入的 TokenStream 并返回一个新的 TokenStream
    //     input
    // }
    /////// 实现一个自定义derive宏
    Pancake::hello_macro();
}

pub fn t19_advanced_feature() {
    println!("---------- Running chapter 19 advanced feature examples...");
    t_dereference_raw_pointer();
    t_use_split_at_mut_from_std();
    let mut vec = vec![1, 2, 3, 4, 5];
    t_use_unsafe_to_implement_split_at_mut(&mut vec[..], 3);
    t_call_c();
    t_use_static_variable();
    t_advanced_trait();
    t_operator_overload();
    //t_sameFuncName_between_Trait_and_Struct();
    t_supertrait();
    t_implement_external_trait_on_external_type_using_newtype_mode();
    t_advanced_type();
    t_advanced_function_closure();
    t_macro();
}   