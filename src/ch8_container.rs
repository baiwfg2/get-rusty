 #[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

/* 当Rust开发者们提到“字符串”时，他们通常指的是String与
字符串切片&str这两种类型，而不仅仅只是其中的一种。虽然本节会
着重介绍String，但是这两种类型都广泛地被应用于Rust标准库中，
并且都采用了UTF-8编码
*/
fn String_op() {
    let mut s = String::from("hello");
    s.push_str(", world");
    s.push('!');
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = String::from("world");
    // + converted to : fn add(self, s: &str) -> String
    let s3 = s1 + " " + &s2; // note: s1 has been moved here and can no longer be used
    println!("s3: {}", s3);
    //println!("s1: {}", s1); // value borrowed here after move. consider cloning the value if the performance cost is acceptable:　s1.clone() + xxx
    let s4 = format!("{}-{}", s3, s2); // format! does not take ownership of any of its parameters and returns a new String
    println!("s4: {}", s4);

    /* P226
    error[E0277]: the type `str` cannot be indexed by `{integer}`
  --> src/ch8_container.rs:23:15
   |
   |     let h = s[0];
   |               ^ string indices are ranges of `usize`
   |
   = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
   = note: you can use `.chars().nth()` or `.bytes().nth()`
     */
    //let h = s[0];

    let hello = "Зд";
    let helloStr = &hello[0..4]; // 这里的4是字节索引，而不是字符索引，因为每个俄语字符占用2个字节
    println!("Russian helloStr: {}", helloStr);
    // 错误的range 会导致 crash
    //let helloStr2 = &hello[0..3]; //  panicked . byte index 3 is not a char boundary; it is inside 'д' (bytes 2..4) of `Здравствуйте

    for c in hello.chars() {
        println!("char: {}", c);
    }
    for b in hello.bytes() {
        println!("byte: {}", b);
    }
}

fn Hashmap_op() {

}

pub fn t8_container() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    for i in row { // `row` moved due to this implicit call to `.into_iter()`
        println!("cell: {:?}", i);
    }
    //println!("one cell: {:?}", row[0]); // value borrowed here after move

    let v = vec![1, 2];
    for i in &v {
        println!("i = {}", i);
    }

    let mut v2 = vec![1, 2];
    for i in &mut v2 {
        *i += 1; // dereference to change the value in the vector
    }
    String_op();
    Hashmap_op();
}