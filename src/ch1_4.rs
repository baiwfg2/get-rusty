use core::{slice, str};

fn t_ch1() {
    let guess = 1;
    /* P36
    Rust allows us to shadow the previous value of
guess with a new one. Shadowing lets us reuse the guess variable name rather than
forcing us to create two unique variables, such as guess_str and guess , for example.
We’ll cover this in more detail in Chapter 3, but for now, know that this feature is often
used when you want to convert a value from one type to another type.
     */
    let mut guess = guess;
}

fn t_ch3() {
    let tup = (1, 2.3, 4);
    let (x, y, z) = tup;
    println!("tup elements: {}, {}, {}", x, y, z);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a2 = [3; 5]; // 5个元素，每个元素都是3
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
    for (i, &item) in bytes.iter().enumerate() { // 第二个元素是指向集合中字节的引用，因此使用 &item 来解引用它以获取实际的字节值
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

struct Foo {
    a: String,
    b: i32,
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
        // move occurs because `r3` has type `&mut std::string::String`, which does not implement the `Copy` trait
        let a = r3; //  value moved here
        //println!("{}", r3); // value borrowed here after move
    }
    let ref r4 = &mut s;
    println!("{}", r4); // r3 already out of scope

    let mut s2 = String::from("hello");
    let r5 = &s;
    // if uncommeted, report: cannot borrow `s` as mutable because it is also borrowed as immutable
    // let r6 = &mut s;
    println!("r5: {}", r5);
    let s3 = dangle();

    {
        let mut foo = Foo { a: String::from("1"), b: 2 };
        let r_foo = &foo;
        //let a = r_foo.a; // cannot move out of `r_foo.a` which is behind a shared reference (String does not implement the `Copy` trait)
    }

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
    let str_literal: &str = "hello world";
    let w3 = first_word_with_slice(&str_literal[..]);
    let w4 = first_word_with_slice(str_literal);
    println!("first_word_with_slice: {}, {}, {}", w1, w2, w3);
}

fn t_handle_ifelse_and_match() {
    let config_max = Some(3u8);
    match config_max {
        /*
        To satisfy the match expression, we have to add _ => () after processing
        just one variant, which is annoying boilerplate code to add.
         */
        Some(max) => println!("max is configured to be {} ", max),
        _ => (),
    }

    /*
    Using if let means less typing, less indentation, and less boilerplate code. However,
you lose the exhaustive checking match enforces that ensures you aren’t forgetting to
handle any cases. Choosing between match and if let depends on what you’re doing
in your particular situation and whether gaining conciseness is an appropriate trade-off
for losing exhaustive checking.
     */
    if let Some(max) = config_max {
        println!("max is configured to be {} ", max);
    }

    let foo = Some(Foo { a: String::from("hello"), b: 42 });
    ///// 这样写很丑(P150)
    let state = if let Some(f) = foo {
        f
    } else {
        return;
    };

    // 可改用 let else (不可再用foo，已经move了)
    let Foo { a, b } = state else {
        return;
    };
}

pub fn t_ch1_ch4() {
    //println!(">>> \033[32mt_ch1_ch4\033[0m"); // 不支持八进制转义
    println!(">>> \x1b[32mt_ch1_ch4\x1b[0m");
    t_ch3();
    ch4_ownership();
}