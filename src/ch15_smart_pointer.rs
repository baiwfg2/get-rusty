use std::ops::Deref;
//use std::ops::Drop;
use std::rc::{Rc, Weak};
use std::cell::{Ref, RefCell};
use std::vec;
use std::collections::HashMap;

/*
引用是只借用数据的指针， 但多数智能指针本身就拥有它们指向的数据
*/

// enum List {
//     Cons(i32, List), //  recursive type `List` has infinite size
//     Nil,
// }

enum BoxList {
    Cons(i32, Box<BoxList>),
    Nil,
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

// RefCell<T> 代表了其持有数据的唯一所有权，且rust只会在运行时检查borrow 规则
// 和Rc<T> 类似，只能用于单线程场景
// P477 有关于如何选择 Box, Rc, or RefCell

// Rc 允许多个所有者持有同一份数据，但只提供不可变访问；若在 Rc里放RefCell,则可
//  拥有多个所有者且能够进行修改的值了

#[derive(Debug)]
enum RefCellList {
    Cons(Rc<RefCell<i32>>, Rc<RefCellList>),
    Nil,
}

#[derive(Debug)]
enum CycledList {
    Cons(i32, RefCell<Rc<CycledList>>),
    Nil,
}

// 取回第二个元素
impl CycledList {
    fn tail(&self) -> Option<&RefCell<Rc<CycledList>>> {
        match self {
            CycledList::Cons(_, s) => Some(s),
            CycledList::Nil => None,
        }
    }
}

struct MyBox<T>(T); // tuple struct，可以只指定类型，不指定字段名。访问时用 self.0 , self.1

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 支持解引用操作
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("[CH15-SMART-POINTER] hello, {}", name);
}

struct CustomSmartPointer {
    dat: String,
}

impl Drop for CustomSmartPointer {
    // 相当于析构函数，执行顺序也同析构函数的逻辑
    fn drop(&mut self) {
        println!("drop custom smartpointer with data `{}`", self.dat);
    }
}

fn _Rc_refcnt_test() {
    println!("------------ [Rc_refcnt_test]");
    let a3 = Rc::new(RcList::Cons(5,
        Rc::new(RcList::Cons(10,
            Rc::new(RcList::Nil)))));
    println!("counter after after creating a3, {}", Rc::strong_count(&a3));
    let b3 = RcList::Cons(3, Rc::clone(&a3));
    println!("counter after after creating b3, {}", Rc::strong_count(&a3));
    {
        let c = RcList::Cons(4, Rc::clone(&a3));
        println!("counter after after creating c, {}", Rc::strong_count(&a3));
        // Rc<T>'s Drop will decrement refcnt
    }
    println!("counter after after c out of scope, {}", Rc::strong_count(&a3));
}

fn _RefCell_test() {
    println!("------------ [RefCell_test]");
    let x = 5;
    //let y = &mut x; // cannot borrow immutable local variable x as mutable

    use RefCellList::{Cons, Nil};
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // 为了确保b,c 能够同时指向a ，a 需封装到 Rc中
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("a after:{:?}", a);
    println!("b after:{:?}", b);
    println!("c after:{:?}", c);
    // P486 通过使用RefCell,List保持了表面上的不可变状态，并能够在必要时借RefCell提供的方法 来修改其
    //   内部的数据

    // https://doc.rust-lang.org/std/cell/
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    // Create a new block to limit the scope of the dynamic borrow
    {
        let mut map = shared_map.borrow_mut();
        map.insert("africa", 1);
        map.insert("kyoto", 2);
        map.insert("piccadilly", 3);
        map.insert("marbles", 4);
    } // 可变借用在这里结束

    // Note that if we had not let the previous borrow of the cache fall out
    // of scope then the subsequent borrow would cause a dynamic thread panic (BorrowError).
    // This is the major hazard of using `RefCell`.
    // 获得了在不可变引用下修改数据的能力，但需要承担运行时借用检查失败的风险
    let total: i32 = shared_map.borrow().values().sum();
    println!("hash value sum: {total}");
}

// Page488
fn CycledRef_test() {
    println!("--------------- [CycledRef_test]");
    use CycledList::{Cons, Nil};
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a inital rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // 修改a的值，让a指向b
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2

    // Error: thread 'main' has overflowed its stack
    //  Debug trait will recursively print
    // println!("a next item = {:?}", a.tail());
}
// Page 479
pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker { messenger, value: 0, max: max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error, you are over quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Error, you are over 90%");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Error, you are over 75%");
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn weak_count_test() {
    println!(">>> weak_count_test");
    let leaf = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf strong:{}, weak:{}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 2,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("branch strong:{}, weak:{}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("leaf strong:{}, weak:{}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
    println!("after branch out of scope, leaf parent:{:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong:{}, weak:{}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}

// 避免循环引用
fn WeakPointer_test() {
    println!("--------------- [WeakPointer_test]");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent={:?}", leaf.parent.borrow()); // leaf parent=(Weak)
    println!("leaf parent={:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("after making leaf.parent point to branch, leaf parent:{:?}", leaf.parent.borrow().upgrade());
    weak_count_test();
}

pub fn t15_smart_pointer() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); // 数值和引用是两种不同的类型，不能直接比较，必须解引用运算符

    let z = Box::new(x);
    assert_eq!(5, *z); // 指向一个x值的装箱指针

    let w = MyBox::new(x);
    assert_eq!(5, *w);

    let ss = MyBox::new(String::from("rust"));
    // 自动将MyBox引用转成 &String, 而标准库的String也实现了Deref可转成 &str，所以才可这样传递
    // 如果MyBox 未实现Deref，则不得不这样调用： hello(&(*ss)[..])
    hello(&ss);

    println!("[DROP TRAIT]");
    let c = CustomSmartPointer { dat: String::from("abc") };
    drop(c); // 调std::mem::drop 可提前释放
    let c = CustomSmartPointer { dat: String::from("abc2") };

    use BoxList::{Cons, Nil};

    let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); //  value used here after move
    // Box<T> 无法让两个列表同时持有另一列表的所有权。可用Rc<List>

    println!("[using Rc.clone]");
    let a2 = Rc::new(RcList::Cons(5,
                Rc::new(RcList::Cons(10,
                    Rc::new(RcList::Nil)))));
    // d, e 共享Rc<RcList>的数据的所有权,只有当引用计数减少到0，才会被真正清理
    // Rc::clone won't execute deep copy
    let d = RcList::Cons(3, Rc::clone(&a2));
    let e = RcList::Cons(4, Rc::clone(&a2));

    _Rc_refcnt_test();

    _RefCell_test();
    CycledRef_test();

    WeakPointer_test();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        // sent_msg: Vec<String>,
        sent_msg: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            // MockMessenger { sent_msg: vec![] }
            MockMessenger { sent_msg: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // `self` is a `&` reference, so the data it refers to cannot be
            //      borrowed as mutable
            // 接收了不可变的self引用 ，所以无法修改 MockMessenger的内容来记录消息
            //self.sent_msg.push(String::from(msg));

            // self 仍然是不可变引用，与send原型保持一致
            // 调用了 RefCell的borrow_mut来获取 Vec<String> 的可变引用
            self.sent_msg.borrow_mut().push(String::from(msg));

            /* 出现 already borrowed: BorrowMutError
            let mut one_borrrow = self.sent_msg.borrow_mut();
            let mut two_borrow = self.sent_msg.borrow_mut();
            one_borrrow.push(String::from(msg));
            two_borrow.push(String::from(msg));
            */
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_msger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_msger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_msger.sent_msg.borrow().len(), 1);
    }
}