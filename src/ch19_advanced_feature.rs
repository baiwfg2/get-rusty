
use std::collections::linked_list;
use std::fmt;
use std::ops::Add;
use std::rc::Rc;
use std::slice;
use std::sync::atomic::AtomicU64;
use std::sync::Mutex;
use std::vec;

// 因为 proc_macro crate 只能在 Rust 的 proc_macro 类型 crate（即宏 crate）中使用，
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
        *r2 += 1;
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

///////////////////////////////////////
static mut COUNTER: u32 = 0;

// Eng ver P594 说得把此函数也标记为 unsafe 更好些
fn add_to_count(inc: u32) {
    // If no unsafe, report: use of mutable static is unsafe and requires unsafe function or block
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
    add_to_count(3);
    unsafe {
        // error: shared reference to mutable static
        //println!("COUNTER: {}", COUNTER);
        /*
        &COUNTER              // 创建普通引用
        &mut COUNTER          // 创建可变引用
        &raw const COUNTER    // 创建不可变原始指针 （const不能省，明确要求程序员指明）
        &raw mut COUNTER      // 创建可变原始指针
         */
        println!("COUNTER by raw pointer deference from mutable static var: {}", *(&raw const COUNTER));
    }
    // Eng ver P596 可使用 Miri 在运行时执行unsafe 代码的检查
}

fn t_unsafe_trait() {
    // a trait is unsafe when at least one of its methods has some invariants that the compiler can't verify
    unsafe trait Foo {
        // methods
    }

    unsafe impl Foo for i32 {
        // implement methods
    }
}

/////////////////////////////////////
fn t_advanced_trait() {
    struct Counter {}
    impl Iterator for Counter {
        // 这叫关联类型，可以定义出包含某些类型的trait，而必须在实现前确定它们的具体类型是什么 (P603)
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            Option::Some(1)
        }
    }

    // 关联类型与generic trait有区别，后者要在实现trait时标注具体类型
    /*
    关联类型适用于"一个类型只能有一种特定关系"的场景，而泛型适用于"一个类型可以处理多种不同类型"的场景。
    Iterator 的设计哲学是：每个具体类型的迭代器，只能产生一种特定类型的元素，所以用关联类型更合适。
     */
    trait MyIterator<T> {
        fn next(&mut self) -> Option<T>;
    }
}


////////////// 运算符重载和默认泛型参数 ////////////////////
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Add 有泛型参数，因为可以为Point实现 多种不同类型的加法，所以比用 type Item好
impl Add for Point {
    // If ommitted, report: not all trait items implemented, missing: `type Output`(在Add trait 中定义)
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn t_operator_overload_with_default_trait_generic_parameter() {
    assert!(Point { x: 1, y: 2 } + Point { x: 3, y: 4 } == Point { x: 4, y: 6 });
}

mod same_name_between_methods {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    trait Animal {
        fn baby_name() -> String;
    }
    struct Dog;
    impl Dog {
        fn baby_name() -> String {
            String::from("Dog's function")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("trait implementation baby_name")
        }
    }

    pub fn t_sameFuncName_between_Trait_and_Struct() {
        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person); // 当是方法时，直接 trait::xxx 是可行的
        person.fly(); // 等价于 Human::fly(&person)

        println!("A baby dog is called a: {}", Dog::baby_name());
        // 由于同名的函数不是方法 ，编译器无法确定调用哪个，需用完全限定语法（full qualified syntax）显式指定 (P611)
        println!("A baby dog is called a: {}", <Dog as Animal>::baby_name());
    }
}

///////////// 超级 trait ////////////////////
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

// Point必须实现Display trait , 因为 OutlinePrint 继承自 Display
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
            // Wrapper 是一个元组结构体 (tuple struct)，其0号是Vec<T>
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("Hello"), String::from("World")]);
    println!("w: {}", w);
    // 这里的问题在于，如果要把Wrapper 作为 Vec<T> 来使用，就得实现Deref trait（避免手动实现所有委托方法）
}

mod advanced_type {
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
            // match 分支必须返回相同类型
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue, // continue的返回类型是 ! 当失败时，不会给guess赋值
            };
            break;
        }
        // Option.wrap也类似这种情况，None 分支有panic! ，属于 ! 类型
    }

    fn t__dynamic_sized_type() {
        // str类型是不能确定大小的，但 &str 却可以，底层会存储数据起始位置和长度
        // let s1: str = "hello"; //  the size for values of type `str` cannot be known at compilation time
        trait foo {}
        let s1: Box<dyn foo>;
        let s2: std::rc::Rc<dyn foo>;
        let s3: &dyn foo;
        // 以上三种类型都是动态大小类型（DST），它们的大小在编译时未知

        fn generic<T>(t: T) {} // T 默认实现了 Sized trait (编译时可计算出类型所占大小)
        fn generic2<T: ?Sized>(t: &T) {} // ?Sized 表示 T 可以是动态大小类型也可以是固定大小类型,这个语法只能用在Sized trait上
        // 如果没有写 &T ,则报：doesn't have a size known at compile-time
    }

    pub fn t_advanced_type() {
        t__type_alias();
        t__never_type();
        t__dynamic_sized_type();
    }
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

    fn _accept_both_fn_and_closure() {  //P625
        let list_of_numbers = vec![11, 2, 3];
        // map 就接受实现了 Fn trait 的闭包或函数指针
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
        assert_eq!(list_of_strings, vec!["11", "2", "3"]);
        let list_of_strings2: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
        assert!(list_of_strings2 == vec!["11", "2", "3"]);
    }
    _accept_both_fn_and_closure();

    #[derive(Debug, PartialEq)]
    enum Status {
        Value(u32),
        Stop,
    }
    /*
    Recall from “Enum values” in Chapter 6 that the name of each enum variant that we
    define also becomes an initializer function. We can use these initializer functions as
    function pointers that implement the closure traits, which means we can specify the
    initializer functions as arguments for methods that take closures
    */
    // 使用Status::Value 构造函数来创建 Status::Value(0), Status::Value(1) 等
    let list_of_statuses: Vec<Status> = (0u32..3).map(Status::Value).collect();
    assert_eq!(
        list_of_statuses,
        vec![Status::Value(0), Status::Value(1), Status::Value(2)]
    );
    // P625: 使用闭包trait 的泛型来编写函数，可同时处理闭包和普通函数

    // return closure
    // error:  expected a type, found a trait(无法推断多大空间来存储此返回的闭包)
    // fn returns_closure() -> Fn(i32) -> i32 {
    //     |x| x + 1
    // }

    fn returns_Box_dyn_closure() -> Box<dyn Fn(i32) -> i32> {  // P627
        Box::new(|x| x + 1)
    }

    mod returned_closure_with_different_opaque_type {

        // Eng ver P620, 中文版没有 !!!!!
        fn returns_closure() -> impl Fn(i32) -> i32 {
            |x| x + 1
        }

        fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
            move |x| x + init
        }

        fn returns_closure2() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }

        fn returns_initialized_closure2(init: i32) -> Box<dyn Fn(i32) -> i32> {
            Box::new(move |x| x + init)
        }

        pub fn test() {
            // 尽管这两函数都返回相同类型的闭包，但底层的opaque type不一样，报：expected opaque type, found a different opaque type
            //let handlers = vec![returns_closure(), returns_initialized_closure(5)];

            // use trait object can work
            let handlers = vec![returns_closure2(), returns_initialized_closure2(5)];
            for handler in handlers {
                let output = handler(5);
                println!("returned_closure_with_different_opaque_type Output: {}", output);
            }
        }
    }
    returned_closure_with_different_opaque_type::test();

}

/////////////////////////// 宏 //////////////////////////
/// extension material: https://lukaswirth.dev/tlborm/syntax-extensions/source-analysis.html
// 用于通用元编程的 declarative macro
#[macro_export]
macro_rules! myvec {
    // Within $() is $x:expr , which matches any Rust expression and gives the expression the name $x .
    // The * specifies that the pattern matches zero or more of whatever precedes the *
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            /* $()* is generated for each part that matches $() in the
                pattern zero or more times depending on how many times the pattern matches. The $x
                is replaced with each expression matched.*/
            $(
                temp_vec.push($x);
            )*      // if * omitted, report: expected one of: `*`, `+`, or `?`
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

// if omitted, report:  expected trait, found derive macro `HelloMacro`
use hello_macro::HelloMacro;
// if omited, report: cannot find derive macro `HelloMacro` in this scope
use hello_macro_derive::HelloMacroDerive;

// procedural macro 用于更复杂的元编程任务
/// 之所以不想手动让类型实现trait，是因为可能要为很多类型实现同样的trait，且rust没有反射机制，无法在运行时查找类型名(Eng ver P628)
#[derive(HelloMacroDerive)]
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

    // ------------------ attribute macro 示例
    // #[proc_macro_attribute]
    // The first is for the contents of the attribute: the GET, "/" part.
    //   The second is the body of the item the attribute is attached to: in this case, fn index() {} and the rest of the function’s body
    // pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
    // #[route(GET, "/")]
    // fn index() {}

    // ------------------ function-like macro 示例
    // let sql = sql!(SELECT * FROM posts WHERE id = 1;);
    // #[proc_macro]
    // pub fn sql(input TokenStream) -> TokenStream {}

}

pub fn t19_advanced_feature() {
    println!("---------- Running chapter 19 advanced feature examples...");
    t_dereference_raw_pointer();
    t_use_split_at_mut_from_std();
    let mut vec = vec![1, 2, 3, 4, 5];
    t_use_unsafe_to_implement_split_at_mut(&mut vec[..], 3);
    t_call_c();
    t_use_static_variable();
    t_unsafe_trait();
    t_advanced_trait();
    t_operator_overload_with_default_trait_generic_parameter();
    same_name_between_methods::t_sameFuncName_between_Trait_and_Struct();
    t_supertrait();
    t_implement_external_trait_on_external_type_using_newtype_mode();
    advanced_type::t_advanced_type();
    t_advanced_function_closure();
    t_macro();
}