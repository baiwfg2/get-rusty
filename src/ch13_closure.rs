// 将闭包和闭包结果绑在一起，这样在合适的逻辑下，耗时闭包只会调用一次，下次直接取缓存值
struct Cacher<T> where T: Fn(u32) -> u32 {
    calc: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calcu: T) -> Cacher<T> {
        Cacher { calc: calcu, value: None, }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calc)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // 返回一个不可变引用的迭代器
    for val in v1_iter { // 这里没要求v1_iter可变，是因为循环取得了 v1_iter的所有权并在内部使得它可变了 ??
        println!("iter got: {}", val);
    }
    // 前面 v1_iter 已经moved了
    //let total: i32 = v1_iter.sum(); //  报：value used here after move (P407)

    // 如果没有调collect，则不会消耗迭代器，闭包也就不会被调
    let v2: Vec<_> = v1.iter().map(|x| x+ 1).collect();
    if v2 != vec![2, 3, 4] {
        panic!("oops");
    }

    // 2. into_iter() - 获取所有权的迭代器 ； 返回 T 类型的元素，消耗原集合，转移所有权； 适用于数据转换和消费
    let v3 = vec![1, 2, 3];
    for val in v3.into_iter() {
        println!("into_iter got: {}", val); // val 类型是 i32，获取了所有权
    }
    // println!("{:?}", v3); // 错误：v3 已被移动

    // into_iter() 用于转换和消费数据
    let v4 = vec![String::from("hello"), String::from("world")];
    let v5: Vec<String> = v4.into_iter()
        .map(|s| s.to_uppercase()) // 可以直接修改 s，因为拥有所有权
        .collect();
    assert_eq!(v5, vec![String::from("HELLO"), String::from("WORLD")]);
    // println!("{:?}", v4); // 错误：v4 已被移动

    // 3. iter_mut() - 返回可变引用的迭代器
    let mut v6 = vec![1, 2, 3];
    for val in v6.iter_mut() {
        *val *= 2; // val 类型是 &mut i32，可以修改原值
    }
    assert_eq!(v6, vec![2, 4, 6]);

    // iter_mut() 用于就地修改集合元素
    let mut v7 = vec![String::from("hello"), String::from("world")];
    v7.iter_mut().for_each(|s| s.push_str("!"));
    println!("iterator:: after iter_mut modification: {:?}", v7); // ["hello!", "world!"]
}

pub fn t13_closure() {
    let example_closure = |x| x;
    // before calling(auto-referring type), there's be error complaining cannot identify x type
    let n = example_closure(5);

    let x = 4;
    //let equal_to_x = |z| z == x; // type must be known at this point
    // 不可变地借用了x，并实现了Fn trait(所有闭包都自动实现FnOnce)
    let equal_to_x2 = |z: i32| z == x;
    println!("after reading by closure, x is still usable: {}", x);

    let mut y = 4;
    // 如果不设置闭包为mut，则报：// cannot borrow `equal_to_y` as mutable, as it is not declared as mutable
    // 理解为 y是被闭包借用并修改了状态,相当于 self.xxx 发生了改变，self 自己也必须是mut
    let mut equal_to_y = |z: i32| -> i32 {y += 1; y};
    println!("after modified by closure, y is usable: {}", equal_to_y(0));

    let x = vec![1, 2, 3];
    // move 强制闭包获取值的所有权
    let equal_to_x_withMove = move |z: Vec<i32>| z == x;
    // println!("cannot use x here: {:?}", x);
    iterator();
}

#[derive(PartialEq, Debug)] // assert_eq needs

struct Shoe {
    size: u32,
    style: String,
}

/// collect shoes that match some condition
///
/// # examples
///
/// ```
/// let shoes = vec![
/// Shoe { size: 10, style: String::from("sneaker")},
/// Shoe { size: 13, style: String::from("sandal")},
/// Shoe { size: 10, style: String::from("boot")},
/// ];
/// let res = shoes_in_my_size(shoes, 10);
/// assert_eq!(
/// res,
/// vec![
///     Shoe { size: 10, style: String::from("sneaker")},
///     Shoe { size: 10, style: String::from("boot")},
/// ]
/// );
/// ```
fn shoes_in_my_size(shoes: Vec<Shoe>, shoeSize: u32) -> Vec<Shoe> {
    // into_iter 创建可以获取动态数组所有权的迭代器
    shoes.into_iter().filter(|s| s.size == shoeSize)
        .collect()
}

///////////////////// 创建自定义的迭代器(P410)
struct Counter {
    cnt: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { cnt:0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.cnt += 1;
        if self.cnt < 6 {
            Some(self.cnt)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_IteratorFilter() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker")},
            Shoe { size: 13, style: String::from("sandal")},
            Shoe { size: 10, style: String::from("boot")},
        ];
        let res = shoes_in_my_size(shoes, 10);
        assert_eq!(
            res,
            vec![
                Shoe { size: 10, style: String::from("sneaker")},
                Shoe { size: 10, style: String::from("boot")},
            ]
        );
    }

    #[test]
    fn test_createIteratorForStruct() {
        // 必须设置为可变，因为next 改变了迭代器内部用来记录序列位置的状态
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_usingCombinedIteratorFunc() {
        // 这些方法都是标准库 Iterator 里的默认实现
        // zip 只会产生4对值，在任意一个迭代器返回None时结束迭代
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                     .map(|(a, b)| a * b)
                                     .filter(|x| x % 3 == 0)
                                     .sum();
        assert_eq!(18, sum);
    }
}