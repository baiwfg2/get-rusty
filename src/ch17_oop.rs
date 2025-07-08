
pub struct Post {
    /*
    什么时候用 dyn State？
    当你需要在运行时决定使用哪个具体类型时
    当你需要存储不同类型的对象在同一个集合中时
    当你需要多态行为时

    用Box 的原因是trait 对象大小不一样，编译时需要知道确切的大小，所以对象要放在堆上
     */
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // as_ref()后得到的是 Option<&Box<dyn State>>，需要的只是Option中值的引用，而不是它的所有权
        // Page 551: 如果不用as_ref，会报错，因为不能将state从&self 中移出
        // Post 方法保证了总会有一个有效的Some值，所以不用担心unwrap panic

        // Page551: 由于解引用转换会依次作用于 & 与 Box，所以最终调用的content来自实现State的具体类型
        self.state.as_ref().unwrap().content(&self)
    }

    // Post的request_review, approve只是对外的接口，与 State trait没有关系，只是函数名故意保持一致而已
    // Page553: 如果Post有更多与State同名的方法，可以使用宏来消除重复
    pub fn request_review(&mut self) {
        // 从option中取值 ，且将option置None. 这样做能使state的值从Post中移出来，而不仅仅是借用它
        // Page548 说不能直接使用 self.state = self.state.request_review(); ???
        //  这可确保Post 无法在我们完成状态转换后再次使用旧的state 值

        /* cursor 解释：
        1. 为什么 Post 里的 state 字段要用 Option<Box<dyn State>>？
            Rust 的所有权系统要求：一个值在同一时刻只能有一个所有者。
            如果 Post 结构体里是这样定义的：state: Box<dyn State>,
            那么你在方法里想这样写：self.state = self.state.request_review();
            这会报错！因为 self.state 已经被借用（或移动）了，不能再直接赋值。
            self.state.take()：把 state 里的 Box<dyn State> 移出来，self.state 变成 None。
            这样你就可以安全地对 s 调用方法，并把新值再放回 self.state。
        3. 为什么不能直接写 self.state = self.state.request_review()？
            因为 self.state 右边的表达式会“借用”或“移动”self.state，
            但左边又要赋值给 self.state，Rust 不允许这样“同时借用和赋值”同一个字段。
         */
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}
trait State {
    // 该方法只能被包裹着当前类型的Box实例调用，它在调用过程中获取Box<Self>的所有权并使旧的状态失效
    // 从而让Post的状态转换为一个新的状态
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // 默认返回空
    // 需要接收Post的引用作为参数，并返回post中某一部分的引用作为结果，因此返回值的生命周期应该与post参数的lifetime相关
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // P553: 返回self,违反对象安全原则，因为trait 无法确定self的具体类型是什么
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // 对于已经处在reivew状态下的文章，发起审批请求并不会改变此文章的当前状态
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        // 因deref coercion ，String引用可自动转换成 &str
        &post.content
    }
}

// P542 Clone trait的clone方法返回了Self，不是对象安全的，所以不能作为动态分发的trait
pub struct Screen {
    // the trait is not dyn compatible because it requires `Self: Sized`
    //   = note: for a trait to be dyn compatible it needs to allow building a vtable
    //pub components: Vec<Box<dyn Clone>>,
}

pub fn t17_oop() {
    ////////// 状态模式设计的一个发贴流程
    /// 在状态模式中，还允许对draft状态执行approve,对pend状态 content，这些是有些多余的，给漏洞留下了空间
    /// 书中另上一种模式（大概率是策略模式）采取了另外一种方法，
    /// 将状态和行为编译成类型，如有 DraftPost, PendPost，
    /// 而这两个struct 根本就没有 content方法，用户没有机会调用；但缺陷是状态流转暴露给用户了
    let mut post = Post::new();
    post.add_text("I love mandy");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I love mandy", post.content());
}