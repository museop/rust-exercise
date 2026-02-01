use proc_macro::TokenStream;
use quote::quote;
use syn::{AttributeArgs, ItemFn, Lit, NestedMeta, parse_macro_input};

#[proc_macro_attribute]
pub fn log_time(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1. 매크로 인자 파싱 (예: #[log_time("MyLabel")])
    // AttributeArgs는 쉼표로 구분된 리터럴 목록을 파싱해줍니다.
    let args = parse_macro_input!(attr as AttributeArgs);

    // 인자에서 라벨 텍스트 추출 (기본값 설정 로직)
    let label = if let Some(NestedMeta::Lit(Lit::Str(lit_str))) = args.first() {
        lit_str.value()
    } else {
        "Function".to_string()
    };

    // 2. 적용할 함수 파싱
    let input_fn = parse_macro_input!(item as ItemFn);

    // 원래 함수의 구성 요소 분리
    let fn_vis = &input_fn.vis; // pub 같은 가시성 
    let fn_sig = &input_fn.sig; // fn 이름(인자) -> 리턴타입
    let fn_block = &input_fn.block; // { 원래  코드 내용 }

    // 3. 코드 재조립 (quote! 매크로 사용)
    let output = quote! {
        #fn_vis #fn_sig {
            let start_time = ::std::time::Instant::now();

            let result = (|| {
                #fn_block
            })();

            // 시간 측정 출력
            println!("{} executed in: {:?}", #label, start_time.elapsed());

            result
        }
    };

    output.into()
}
