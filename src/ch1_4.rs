use core::{slice, str};

fn t_ch1() {

}

fn t_ch3() {
    let tup = (1, 2.3, 4);
    let (x, y, z) = tup;
    println!("tup elements: {}, {}, {}", x, y, z);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a2 = [3; 5];
    assert!(a2[3] == 3);

    // loop
    let mut num = 3;
    while num != 0 {
        println!("num = {} in a loop", num);
        num -= 1;
    }
}

// Error: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
fn dangle() -> /*&*/String {
    let s = String::from("how to avoid dangle ref ?");
    //&s
    s
}

// 参数是 &str 会更好，因为它可以接受 String 和 &str 两种类型
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // 返回一个与底层数据关联的切片，而不是比如长度本身。因为一旦底层数据改变，长度也就失去了意义
        }
    }
    &s[..]
}

fn first_word_with_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // 返回一个与底层数据关联的切片，而不是比如长度本身。因为一旦底层数据改变，长度也就失去了意义
        }
    }
    &s[..]
}

fn ch4_ownership() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // if uncomment, report:
    //  cannot borrow `s` as mutable more than once at a time
    //let _ref = &s; // cannot borrow `s` as immutable because it is also borrowed as mutable
    println!("{}", r1);

    {
        let r3 = &mut s;
    }
    let ref r4 = &mut s;
    println!("{}", r4); // r3 already out of scope

    let mut s2 = String::from("hello");
    let r5 = &s;
    // if uncommeted, report: cannot borrow `s` as mutable because it is also borrowed as immutable
    // let r6 = &mut s;
    println!("r5: {}", r5);
    let s3 = dangle();

    /////////// slices
    let string1 = String::from("hello world");
    // if omit &, Report: doesn't have a size known at compile-time
    let slice1 = &string1[0..2];
    let slice2 = &string1[..2];
    let slice3 = &string1[6..];
    println!("slices: {}, {}, {}", slice1, slice2, slice3);
    let word = first_word(&string1); // here borrowed as immutable
    // string1.clear(); //  cannot borrow `string1` as mutable because it is also borrowed as immutable
    println!("first_word: {}", word);

    let w1 = first_word_with_slice(&string1[..]);
    let w2 = first_word_with_slice(&string1);
    let str_literal = "hello world";
    let w3 = first_word_with_slice(&str_literal[..]);
    let w4 = first_word_with_slice(str_literal);
    println!("first_word_with_slice: {}, {}, {}", w1, w2, w3);
}

pub fn t_ch1_ch4() {
    println!(">>> \033[32mt_ch1_ch4\033[0m");
    t_ch3();
    ch4_ownership();
}