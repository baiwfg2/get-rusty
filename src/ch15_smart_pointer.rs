use std::ops::Deref;
//use std::ops::Drop;
use std::rc::Rc;

/*
引用是只借用数据的指针， 但多数智能指针本身就拥有它们指向的数据
*/

// enum List {
//     Cons(i32, List), //  recursive type `List` has infinite size
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

struct MyBox<T>(T); // what grammar is it ?

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

    use List::{Cons, Nil};

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

    println!("print Rc<T> refcnt");
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