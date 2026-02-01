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

    let mut builder_fields = Vec::new();
    let mut builder_init = Vec::new();
    let mut setters = Vec::new();
    let mut build_logic = Vec::new();

    for field in fields {
        let field_name = &field.ident;
        let field_ty = &field.ty;

        // 1. 속성 파싱: #[builder(each = "name")] 찾기
        let mut each_name = None;
        for attr in &field.attrs {
            if attr.path.is_ident("builder") {
                if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                    for nested in meta_list.nested {
                        if let NestedMeta::Meta(Meta::NameValue(nv)) = nested {
                            if nv.path.is_ident("each") {
                                if let Lit::Str(lit_str) = nv.lit {
                                    each_name = Some(format_ident!("{}", lit_str.value()));
                                }
                            }
                        }
                    }
                }
            }
        }

        // 2. 타입 분석 및 로직 분기
        // (A) Vec이면서 each 속성이 있는 경우
        if let (Some(inner_ty), Some(each_ident)) = (get_inner_type_of_vec(field_ty), each_name) {
            // 빌더 필드: Vec은 Option으로 감싸지 않고 바로 사용 (초기값: 빈 벡터)
            builder_fields.push(quote! { #field_name: #field_ty });
            builder_init.push(quote! { #field_name: std::vec::Vec::new() });

            // Setter: push 방식 (이름은 each_ident 사용)
            setters.push(quote! {
                pub fn #each_ident(mut self, input: #inner_ty) -> Self {
                    self.#field_name.push(input);
                    self
                }
            });

            // Build: 그대로 전달 (이미 Vec 형태임)
            build_logic.push(quote! {
                #field_name: self.#field_name
            });
        }
        // (B) Option인 경우 (기존 로직)
        else if let Some(inner_ty) = get_inner_type_of_option(field_ty) {
            builder_fields.push(quote! { #field_name: std::option::Option<#inner_ty> });
            builder_init.push(quote! { #field_name: std::option::Option::None });

            setters.push(quote! {
                pub fn #field_name(mut self, input: #inner_ty) -> Self {
                    self.#field_name = std::option::Option::Some(input);
                    self
                }
            });

            build_logic.push(quote! { #field_name: self.#field_name });
        }
        // (C) 일반 필수 필드 (기존 로직)
        else {
            builder_fields.push(quote! { #field_name: std::option::Option<#field_ty> });
            builder_init.push(quote! { #field_name: std::option::Option::None });

            setters.push(quote! {
                pub fn #field_name(mut self, input: #field_ty) -> Self {
                    self.#field_name = std::option::Option::Some(input);
                    self
                }
            });

            let err_msg = format!("Field {} is missing", field_name.as_ref().unwrap());
            build_logic.push(quote! {
                #field_name: self.#field_name.ok_or(#err_msg)?
            });
        }
    }

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

// Vec<T>에서 T를 꺼내는 함수
fn get_inner_type_of_vec(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Vec" {
                // 여기만 다름
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}
