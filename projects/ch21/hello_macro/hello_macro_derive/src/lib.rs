extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 型定義の文字列表現を構築する
    // Construct a string representation of the type definition
    let s = input.to_string();

    // 文字列表現を構文解析する
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // implを構築する
    // Build the impl
    let gen = impl_hello_macro(&ast);

    // 生成されたimplを返す
    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    }
}