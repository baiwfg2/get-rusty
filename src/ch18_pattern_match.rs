use std::fmt::Pointer;

fn t1_mix_use_if_and_let() {
    let favorite_color: Option<&str> = Some("blue");
    let is_tuesday = false;
    let age: Result<u8, _> = "18".parse();
    if let Some(color) = favorite_color {
        println!("Using if let: Your favorite color is {}.", color);
    } else if is_tuesday{
        println!("Tuesday is a great day!");
    } else if let Ok(age) = age {
        // 覆盖了同名变量的age只有在{} 后的作用域内有效
        if age > 30 {
            println!("You are over 30 years old.");
        } else {
            println!("You are 30 years old or younger.");
        }
    } else {
        println!("No favorite color, not Tuesday, and age is not valid.");
    }
    // if let的缺陷在于未强制要求处理所有可能的情况，而match 则会强制
}

fn t_while_and_let() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Coordinates: ({}, {})", x, y);
}

fn t_match() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) | Some(2) => println!("Got 50"),
        Some(y) => println!("Matched y: {}", y), // local scope 'y'
        _ => println!("at the end: x is {:?}, y={:?}", x, y),
        
    }
    match y {
        7 ..= 10 => println!("y is between"),
        _ => println!("y is not between"),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn t_structure_binding() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 10, y: 20 };
    let Point { x, y } = p;
    assert_eq!(10, x);
    assert_eq!(20, y);

    match p {
        Point { x: 0, y } => println!("x is zero, y is {}", y),
        Point { x, y: 0 } => println!("y is zero, x is {}", x),
        Point { x, y } => println!("x is {}, y is {}", x, y),
    }

    let msg = Message::ChangeColor(0, 1, 2);
    // P577
    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Write message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change color to r: {}, g: {}, b: {}", r, g, b);
            }
    }
    let ((feet, inches), Point { x, y}) =
        ((5, 11), Point { x: 10, y: 20 });

}

fn use_underscore() {
    println!("-------------- use underscore");
    let s = Some(String::from("Hello"));
    if let Some(_s) = s {
        println!("found a string");
    }
    // Error: value borrowed here after partial move
    //println!("s is now: {:?}", s);

    let s2 = Some(String::from("Hello"));
    // 使用_ 不会对值进行绑定，所以s2 所有权仍然有效
    if let Some(_) = s2 {
        println!("found a string");
    }
    print!("s2 is still: {:?}", s2);
}

fn use_double_dots() {
    println!("-------------- use ..");
    let numbers = (1,2,3,4,5);
    match  numbers {
        (1, 2, ..) => println!("Matched first two numbers: 1 and 2"),
        (.., 4, 5) => println!("Matched last two numbers: 4 and 5"),
        _ => println!("Did not match the specific patterns"),
    }
}

// 在匹配的同时还施加条件
fn use_match_guard() {
    println!("-------------- use match guard");
    let x = Some(5);
    let y = 10;
    match x  {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched n: {:?}", n),
        _ => println!("at the end: x is {:?}, y={:?}", x, y),
    }

    let a = 4;
    let b = false;
    match a {
        // P585, a 要么是4,5,6 ，并且y=true
        4 | 5 | 6 if b => println!("a is 4, 5, or 6 and b is true"),
        _ => println!("no"),
    }
}

fn use_at_binding() {
    println!("-------------- use @ binding");
    enum Message {
        Hello {id: i32},
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            // 使用 @ 绑定变量 id_variable,可以在匹配分支中使用
            println!("Matched id: {}", id_variable);
        },
        Message::Hello { id: 10..=12 } => {
            // println!("Matched id in range 10 to 12, {}", id); // cannot find value `id` in this scope
            // 这个分支未使用 @，无法使用id 变量
            println!("Matched id in range 10 to 12");
        },
        Message::Hello { id } => {
            println!("non-range matching, can use `id` directly, Matched id: {}", id);
        }
    }
}

pub fn t18_pattern_match() {
    println!("Running chapter 18 pattern match examples...");
    t1_mix_use_if_and_let();
    t_while_and_let();
    let point = (1, 2);
    print_coordinates(&point);

    let v = vec!['a', 'b', 'c'];
    for (idx, val) in v.iter().enumerate() {
        println!("Index: {}, Value: {}", idx, val);
    }
    // 普的let 语句也属于模式匹配的一种形式
    let (x, y) = (1, 2);

    let mut setting_value = Some(1);
    let new_setting_value = Some(2);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("cannot overwrite a setting value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("Setting value is now: {:?}", setting_value);

    t_match();
    t_structure_binding();
    use_underscore();
    use_double_dots();
    use_match_guard();
    use_at_binding();
}