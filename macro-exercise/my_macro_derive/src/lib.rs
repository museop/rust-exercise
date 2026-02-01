use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    // 1. 파싱: Struct의 이름, 필드 정보 등을 가져옵니다.
    let input = parse_macro_input!(input as DeriveInput);

    // 구조체 이름 (Command)
    let name = input.ident;

    // 빌더 구조체 이름 생성 (CommandBuilder)
    // format_ident!는 문자열 포맷팅으로 새로운 식별자를 만들 때 씁니다.
    let builder_name = format_ident!("{}Builder", name);

    // 2. 필드 추출 로직 (여기서 핵심 로직을 분리하는 게 좋습니다)
    // 구조체가 아니거나, 필드가 없는 경우 등은 에러 처리가 필요하지만
    // 학습을 위해 "이름 있는 필드를 가진 구조체"라고 가정합니다.
    let fields = if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            fields.named
        } else {
            panic!("Named fields only!");
        }
    } else {
        panic!("Structs only!");
    };

    // 필드 이름과 타입 분리 (반복(Repetition)을 위해 벡터로 만듦)
    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    // 3. 코드 생성 (quote!)
    // quote! 안에서 #(#var)* 구문은 벡터의 요소를 반복해서 출력합니다.
    let expanded = quote! {
        // A. 빌더 구조체 정의
        pub struct #builder_name {
            // 모든 필드를 Option으로 감싸서 초기화 전 상태를 표현
            #(#field_names: std::option::Option<#field_types>),*
        }

        // B. 원래 구조체에 builder() 메소드 추가
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#field_names: std::option::Option::None),*
                }
            }
        }

        // C. 빌더 메소드 구현
        impl #builder_name {
            // 각 필드별 setter 생성
            #(
                pub fn #field_names(mut self, input: #field_types) -> Self {
                    self.#field_names = std::option::Option::Some(input);
                    self
                }
            )*

            // 최종 build 메소드
            pub fn build(self) -> std::result::Result<#name, std::string::String> {
                Ok(#name {
                    #(
                        #field_names: self.#field_names
                            .ok_or(format!("Field {} is missing", stringify!(#field_names)))?
                    ),*
                })
            }
        }
    };

    expanded.into()
}
