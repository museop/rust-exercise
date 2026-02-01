use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(HelloMacro)] // 1. 매크로 이름 등록
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 2. 입력받은 Rust 코드(TokenStream)를 파싱하여 구문 트리(AST)로 변환
    let ast = parse_macro_input!(input as DeriveInput);

    // 3. 실제 로직 수행 (구조체 이름을 뽑아서 코드를 생성)
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident; // 구조체 이름 (식별자)

    // 4. quote! 매크로를 사용하여 생성할 Rust 코드를 작성
    // #name 부분에 변수 값이 들어갑니다 (템플릿 처럼)
    let gen_code = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify!는 코드를 문자열로 바꿔줌
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    // 5. 생성된 코드를 다시 TokenStream으로 변환하여 반환
    gen_code.into()
}
