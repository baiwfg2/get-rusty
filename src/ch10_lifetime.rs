/*
If not annotated with lifetime tag, error will be: expected named lifetime parameter
Because compiler's borrow checker cannot figure out which &str will be returned
如果不标注，则编译器在执行默认的生命周期标注规则后，仍无法判定所有引用的生命周期时，就报错

都用'a表示：两个切片的存活时间不短于'a ，返回值的存活时间也不短于'a
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Page298  当明确了返回类型仅与x 相关， y可以不标注
fn longest_with_one_ref<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// Page330:这个标注意味着 ImportantExcerptWithLifetimeTag 实例的存活时间
//   不能超过存储在part字段中的引用的存活时间 (P300)
struct ImportantExcerptWithLifetimeTag<'a> {
    // annoatate `every ref` when there're refs in struct
    part: &'a str,
}

////////// 方法定义中的生命周期标注 (P304)
impl<'a> ImportantExcerptWithLifetimeTag<'a> {
    // no ref parameter, no ref return
    fn level(&self) -> i32 {
        3
    }

    // (Page 302)Default Rule 1(against input arg): apply annotate to self('a) and ann('b) separately
    // Default Rule 2: When only one input arg, apply its lifetime to all returnValue
    // Default Rule 3: when multiple input arg exist(include &self, &mut self), apply
    //       self's lifetime to all returnValue
    // Otherwise compiler issue errors

    // so in this case every refs can be default annotated without manual annotation
    fn announce_and_return_part(&self, ann: &str) -> &str {
        println!("Attention please:{}", ann);
        self.part
    }
}

use std::fmt::Display;
// use generics, trait constrait, lifetime simutaneously
fn useGenericsAndTraitconstraintAndLifetime<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display {
    println!("[useGenericsAndTraitconstraintAndLifetime] announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn t10_lifetime() {
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let res = longest(s1.as_str(), s2);
    println!("[LIFETIME exercise] the longest string is {}", res);

    ///////// lifetime anti-example (P297)
    let s3 = String::from("abcd");
    let res2;
    {
        let s4 = String::from("xyz");
        res2 = longest(s3.as_str(), s4.as_str());
    }
    // error: borrowed value does not live long enough if calling this. No error if not called
    // println!("the longest string is {}", res2);
    // 返回的引用 的生命周期得是 传入的引用的生命周期中最短的那个，而现在res2却比s4活得更久

    let novel = String::from("call me cshi. some years aog....");
    let first_sentence = novel.split('.').next().expect("ERROR !!");
    let i = ImportantExcerptWithLifetimeTag { part: first_sentence };
    useGenericsAndTraitconstraintAndLifetime(s1.as_str(), s2, 1);
}
