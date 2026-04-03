这是因为 Rust 的核心设计哲学之一：**在调用点（Call Site），借用和所有权转移必须是显式求值的。**

如果你定义了函数 `fn notify(item: &impl Summary)`，它期望的参数类型是“一个引用”。而你的变量 `my_tweet` 则是“一个完整的值（拥有所有权）”。

Rust 故意没有像 C++ 那样做隐式的引用转换（在 C++ 中，如果参数是 `const Tweet&`，你可以直接写 `notify(my_tweet)`，编译器会自动取地址）。

**为什么要强制写成 `notify(&my_tweet)`？**

1. **可见性与清晰度：** 在阅读代码时，当你看到 `notify(&my_tweet)`，你立刻确信：“哦，这里只是把 `my_tweet` 借给这个函数用一下，它不会丢，也不会被修改。”
2. **区分 Move 语义：** 如果 Rust 允许写成 `notify(my_tweet)`，光看这行代码，你无法知道 `my_tweet` 是被**借用（Borrow）**了，还是被**转移（Move / 消耗）**了。你必须去查阅 `notify` 函数的签名才能确认自己后续还能不能继续使用 `my_tweet`。强制写 `&` 避免了这种心智负担。

**唯一例外（方法调用）：**
在调用结构体的方法时（例如 `my_tweet.summarize()`），Rust 有一个叫作**自动引用/解引用（Auto-Deref/Ref）**的机制。如果 `summarize` 的签名是 `&self`，你可以直接写 `my_tweet.summarize()`，而不需要写成 `(&my_tweet).summarize()`。但在调用普通的**独立函数**时，你必须严格匹配类型，手动提供引用 `&` 永远不能省不掉` 出对应的引用。