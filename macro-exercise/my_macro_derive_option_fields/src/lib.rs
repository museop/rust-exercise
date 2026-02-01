use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, Fields, GenericArgument, Lit, Meta, NestedMeta, Path, PathArguments, Type,
    parse_macro_input,
};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let builder_name = format_ident!("{}Builder", name);

    let fields = if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            fields.named
        } else {
            panic!("Named fields only!");
        }
    } else {
        panic!("Structs only!");
    };

    // 생성할 코드 조각들을 모을 벡터
    let mut builder_fields = Vec::new(); // 빌더 구조체 정의용
    let mut builder_init = Vec::new(); // 빌더 초기화용
    let mut setters = Vec::new(); // Setter 메소드용
    let mut build_logic = Vec::new(); // Build 메소드 내부 로직용

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;

        // 1. 타입 분석: Option<T> 인가?
        let (arg_type, is_option) = if let Some(inner_ty) = get_inner_type_of_option(field_ty) {
            (inner_ty, true) // Option<T>라면, 인자는 T 타입이어야 함
        } else {
            (field_ty, false) // 아니라면, 인자는 원래 타입 그대로
        };

        // --- 코드 생성 준비 ---

        // A. 빌더 구조체 필드 (항상 Option으로 감쌈)
        // 사용자가 Option<i32>를 원해도 빌더는 Option<i32>를 가짐 (이중 Option 아님에 주의)
        if is_option {
            builder_fields.push(quote! { #field_name: #field_ty });
        } else {
            builder_fields.push(quote! { #field_name: std::option::Option<#field_ty> });
        }

        builder_init.push(quote! { #field_name: std::option::Option::None });

        // B. Setter 메소드
        // 인자로 arg_type(내부 타입)을 받아서 Some으로 감싸 저장
        setters.push(quote! {
            pub fn #field_name(mut self, input: #arg_type) -> Self {
                self.#field_name = std::option::Option::Some(input);
                self
            }
        });

        // C. Build 로직
        if is_option {
            // Option 타입은 값이 없어도 에러가 아님 (그냥 None 전달)
            build_logic.push(quote! {
                #field_name: self.#field_name
            });
        } else {
            // 일반 타입은 값이 없으면 에러 발생
            let err_msg = format!("Field {} is missing", field_name);
            build_logic.push(quote! {
                #field_name: self.#field_name.ok_or(#err_msg)?
            });
        }
    }

    // 최종 조립
    let expanded = quote! {
        pub struct #builder_name {
            #(#builder_fields),*
        }

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#builder_init),*
                }
            }
        }

        impl #builder_name {
            #(#setters)*

            pub fn build(self) -> std::result::Result<#name, std::string::String> {
                Ok(#name {
                    #(#build_logic),*
                })
            }
        }
    };

    expanded.into()
}

// Option<T>에서 T를 꺼내는 함수
fn get_inner_type_of_option(ty: &Type) -> Option<&Type> {
    // 1. 타입이 Type::Path 인지 확인 (예: std::option::Option<T>)
    if let Type::Path(type_path) = ty {
        // 2. 경로의 마지막 세그먼트가 "Option" 인지 확인
        // (std::option::Option 이면 segments에 [std, option, Option]이 들어있음)
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                // 3. 제네릭 인자(<T>)가 있는지 확인
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    // 4. 첫 번째 인자가 타입인지 확인
                    if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}
