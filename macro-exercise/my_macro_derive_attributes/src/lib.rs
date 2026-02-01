use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Lit, Meta, NestedMeta, parse_macro_input};

// 1. attributes(builder)를 추가하여, #[builder(...)] 속성을 허용한다고 선언
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

    // 필드 정보를 담을 벡터들
    let mut field_names = Vec::new();
    let mut field_types = Vec::new();
    let mut setter_names = Vec::new(); // 실제 메소드 이름 (원래 이름 or rename된 이름)

    // 2. 각 필드를 순회하며 속성 파싱
    for field in fields {
        field_names.push(field.ident.clone().unwrap());
        field_types.push(field.ty.clone());

        // 속성 파싱: #[builder(rename = "new_name")] 찾기
        let mut final_name = field.ident.clone().unwrap(); // 기본값은 필드명

        // field.attrs는 해당 필드 위의 모든 속성(#[...]) 리스트입니다.
        for attr in &field.attrs {
            // 속성 이름이 "builder" 인지 확인
            if attr.path.is_ident("builder") {
                // builder(...) 괄호 안의 내용을 파싱
                if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                    for nested in meta_list.nested {
                        if let NestedMeta::Meta(Meta::NameValue(nv)) = nested {
                            // rename = "..." 패턴인지 확인
                            if nv.path.is_ident("rename") {
                                if let Lit::Str(lit_str) = nv.lit {
                                    // 찾았다! 새로운 이름 적용
                                    final_name = format_ident!("{}", lit_str.value());
                                }
                            }
                        }
                    }
                }
            }
        }
        setter_names.push(final_name);
    }

    // 3. 코드 생성 (이제 field_names 대신 setter_names를 사용하는 부분이 생깁니다)
    let expanded = quote! {
        pub struct #builder_name {
            #(#field_names: std::option::Option<#field_types>),*
        }

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#field_names: std::option::Option::None),*
                }
            }
        }

        impl #builder_name {
            // 여기가 핵심 변경점!
            // 메소드 이름은 #setter_names, 내부 필드 접근은 #field_names
            #(
                pub fn #setter_names(mut self, input: #field_types) -> Self {
                    self.#field_names = std::option::Option::Some(input);
                    self
                }
            )*

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
