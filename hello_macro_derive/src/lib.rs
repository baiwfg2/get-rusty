// extern crate proc_macro;
// use crate::proc_macro::TokenStream;

// 以上两行等价于下面这一行
use proc_macro::TokenStream;
// turns syn data structures back into Rust code (https://docs.rs/quote/latest/quote/)
use quote::quote;
// crate parses Rust code from a string into a data structure that we can perform operations on.
use syn;

// 当有人写 #[derive(HelloMacroDerive)] 时，编译器会调用这个函数
#[proc_macro_derive(HelloMacroDerive)]
/* note that the output for our derive macro is also a TokenStream .
The returned TokenStream is added to the code that our crate users write, so when they compile their crate, they’ll
 get the extra functionality that we provide in the modified TokenStream */
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // The parse function in syn takes a TokenStream and returns a
    // DeriveInput struct representing the parsed Rust code.
    let ast = syn::parse(input).unwrap(); // 生产环境下应合理处理错误
    implement_hello_macro(&ast)
}

fn implement_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // The quote! macro lets us define the Rust code that we want to return.
    let gen2 = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // at compile time turns the expression into a string literal, such as "1 + 2" . This
                // is different from format! or println! , macros which evaluate the expression and then
                // turn the result into a String .
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen2.into()
}
