# Rust 매크로(Macro) 학습

## 1단계: 매크로의 필요성과 기본 개념

Rust에서 매크로는 단순히 "기능이 많은 함수"가 아닙니다. 근본적으로 작동하는 시기와 방식이 다릅니다. 이 차이를 이해하는 것이 매크로 정복의 첫 단추입니다.

### 매크로란 무엇인가? (Code that writes code)

함수는 **데이터(값)** 를 입력받아 계산을 수행하지만, 매크로는 **Rust 코드(구문)** 를 입력받아 새로운 Rust 코드를 생성합니다.

이 과정을 **매크로 확장(Macro Expansion)** 이라고 하며, 컴파일러가 코드를 기계어로 번역하기 전에 일어납니다.

### 왜 함수 대신 매크로를 쓸까요? (The Why)

Rust의 함수는 매우 엄격합니다. 매크로는 그 엄격함을 우회하며 유연함을 제공합니다.

#### 가변 인자 (Variable Arguments)

Rust 함수는 정해진 개수와 타입의 인자만 받을 수 있습니다. 하지만 매크로는 인자의 개수가 달라도 됩니다.

예제: `vec!` 매크로

```rust
// 매크로는 인자가 1개든 3개든 상관없습니다.
let v1 = vec![1, 2, 3];
let v2 = vec![1];

// 만약 함수로 구현해야 한다면? (불가능하거나 매우 복잡함)
// fn make_vec(arg1: i32, arg2: i32, ...) -> 불가능
```

#### 메타정보 접근 (변수 이름 출력 등)

함수는 인자로 넘어온 값만 알 수 있지만, 매크로는 코드를 텍스트처럼 읽기 때문에 변수 이름 자체나 파일 위치 등을 알 수 있습니다.

예제: `dbg!` 매크로 vs 함수 
```rust
fn main() {
    let a = 10;

    // 함수는 'a'라는 변수명을 모릅니다. 값 '10'만 받습니다.
    print_function(a); 
    // 출력: 10
    
    // 매크로는 'a'라는 토큰 자체를 해석합니다.
    dbg!(a); 
    // 출력: [src/main.rs:9] a = 10
    // (파일 위치, 변수명 'a', 값 10을 모두 출력)
}

fn print_function(val: i32) {
    println!("{}", val);
}
```

#### 반복되는 코드 자동 생성 (DRY)

비슷한 구조의 코드를 100번 짜야 한다면, 함수로는 해결이 안 될 때가 많습니다(예: 구조체마다 Trait 구현하기). 매크로는 이 코드를 자동으로 "복사-붙여넣기" 해줍니다.

### 매크로 확장 과정 시각화 

매크로가 어떻게 작동하는지 `cargo-expand`라는 도구의 관점에서 보면 다음과 같습니다.

우리가 작성한 코드:
```rust
let v = vec![1, 2];
```

컴파일 타임 (매크로 확장) 코드:
```rust
let v = {
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec
};
```

### 함수 vs 매크로 비교 요약 

| 특징 | 함수 (Functions) | 매크로 (Macros) |
|------|------------------|-----------------|
| 실행 시점 | 런타임 (프로그램 실행 중) | 컴파일 타임 (실행 파일 만들기 전) |
| 입력값 | 값 (Value), 변수 | 코드 (Tokens, Syntax Tree) |
| 인자 개수 | 고정됨 (엄격) | 가변적 (유연) |
| 호출 형태 | name(...) | name!(...) (대부분 !가 붙음) |
| 디버깅 | 쉬움 (흐름 추적 가능) | 어려움 (확장된 코드를 봐야 함) |

## 2단계: 선언적 매크로 (`macro_rules!`)

선언적 매크로는 Rust에서 가장 흔하게 접하는 매크로 작성 방식입니다. 문법이 마치 match 표현식과 비슷해서 **"패턴 매칭을 하는 코드 생성기"** 라고 생각하면 이해가 빠릅니다.

### 기본 구조: 매크로는 `match`와 유사

`match`가 값에 따라 분기한다면, `macro_rules!`는 **코드의 생김새(패턴)** 에 따라 분기합니다.

```rust
macro_rules! 매크로이름 {
    // (패턴) => { 코드로 변환 };
    ($x:expr) => {
        println!("표현식을 받았습니다: {}", $x);
    };
    () => {
        println!("아무것도 안 받았습니다.");
    };
}
```

여기서 가장 중요한 것이 바로 `$x:expr` 같은 부분입니다. 이를 **메타변수(Metavariable)** 라고 부릅니다.

### 메타변수와 지정자 (Designators)

매크로가 사용자의 코드를 "캡처"하기 위해서는 어떤 종류의 코드인지 명시해줘야 합니다. `$이름:지정자` 형식을 사용합니다.

자주 쓰는 지정자 3대장:

1. `expr` (Expression): 값을 반환하는 모든 식 (`1+1`, `"hello"`, `func()`, `x`)
2. `ident` (Identifier): 식별자 (`변수명`, `함수명`, `타입명` 등)
3. `ty` (Type): 타입 (`i32`, `String`, `Option<T>`)

예제: 디버그 매크로
```rust
// 1. 매크로 정의
macro_rules! show_me {
    // 변수명($name)과 값(&val)을 받아서 출력하는 패턴
    ($name:ident, $val:expr) => {
        println!("변수 이름: {}, 값: {}", stringify!($name), $val); // 
    };
}

fn main() {
    let my_num = 100;
    // 호출 시: 식별자(my_num)와 값(my_num)을 전달
    show_me!(my_num, my_num);
}
```

`stringify!`는 토큰을 문자열로 바꿔주는 내장 매크로입니다.

### 반복 (Repetition): 매크로의 꽃 

매크로가 강력한 이유는 인자를 1개든 100개든 받을 수 있기 때문입니다. 반복 문법은 약간 암호 같아 보일 수 있지만, 공식만 알면 쉽습니다.

공식: `$ ( ... ) 구분자 반복횟수`
- `$ ( $x:expr )` : 반복할 패턴을 감쌉니다.
- `,` : 구분자 (콤마로 구분하겠다는 뜻). 생략 가능.
- `*` : 반복 횟수 (`*`는 0번 이상, `+`는 1번 이상, `?`는 0 또는 1번)

예제: `vec!` 매크로 흉내 내기 (`my_vec!`)

```rust
macro_rules! my_vec {
    // 1. 패턴 매칭: 표현식($x)이 콤마(,)로 구분되어 0번 이상(*) 반복됨
    ( $( $x:expr ),* ) => {
        {
            #[allow(unused_mut)]
            let mut temp_vec = Vec::new();
            // 2. 코드 확장: 캡처한 $x에 대해 반복 생성
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    // 인자가 0개여도 되고
    let v1: Vec<i32> = my_vec![];
    // 인자가 여러 개여도 됩니다.
    let v2 = my_vec![1, 2, 3]; 
    
    // v2는 컴파일러에 의해 아래처럼 바뀝니다:
    // {
    //     let mut temp_vec = Vec::new();
    //     temp_vec.push(1);
    //     temp_vec.push(2);
    //     temp_vec.push(3);
    //     temp_vec
    // }
}
```

여기서 `$( temp_vec.push($x); )*` 부분이 핵심입니다. 입력받은 개수만큼 `push` 코드를 자동으로 복사해 줍니다.

### 다중 패턴 (Overloading)
하나의 매크로가 여러 가지 형태를 가질 수 있습니다. `match` 문처럼 `;`로 구분하여 나열하면 됩니다.
```rust
macro_rules! calculator {
    // 패턴 1: 인자가 1개일 때
    ($a:expr) => {
        println!("값 하나만 줌: {}", $a);
    };
    // 패턴 2: 인자가 2개일 때 (add)
    (add $a:expr, $b:expr) => {
        println!("더하기: {}", $a + $b);
    };
}

fn main() {
    calculator!(10);          // 패턴 1 매칭
    calculator!(add 10, 20);  // 패턴 2 매칭 ('add'라는 키워드도 패턴의 일부!)
}
```

## 3단계: 절차적 매크로 기초와 도구 (Procedural Macros)

선언적 매크로(macro_rules!)가 단순히 패턴을 찾아 바꿔치기하는 것이었다면, 절차적 매크로는 코드를 입력받아 내 마음대로 분석하고 재조립하는 "함수"입니다. Rust 컴파일러의 기능을 우리가 빌려 쓰는 것이죠.

### 절차적 매크로의 3대장 (The Holy Trinity)

절차적 매크로를 만들 때 99% 확률로 사용하는 필수 라이브러리 3개가 있습니다. 이들의 역할을 아는 것이 시작입니다.

1. `syn` (Parser): Rust 코드를 우리가 다루기 쉬운 **데이터 구조(구문 트리, AST)** 로 파싱해 줍니다. (예: "이건 구조체고, 이름은 `A`이며, 필드는 `x`, `y`가 있다"라고 분해)
2. `quote` (Code Generator): 데이터 구조를 다시 Rust 코드로 바꿔줍니다. (예: "이 구조체에 메서드를 추가하는 코드를 만들어줘")
3. `proc-macro2`: 원래 Rust 컴파일러 내부 타입들을 래핑하여 외부에서 쓸 수 있게 해주는 접착제입니다.

흐름: Rust 코드 -> `syn`으로 파싱 -> 데이터 구조 -> 로직 처리 -> `quote`로 생성 -> 새로운 Rust 코드

### 가장 중요한 제약 사항: "별도의 크레이트"
절차적 매크로는 반드시 별도의 라이브러리 크레이트에 정의해야 합니다. 그리고 그 크레이트는 매크로 전용이라고 선언해야 합니다.

- `main.rs` (내 프로그램)
- `my_macro/` (매크로 라이브러리) ⬅️ 여기에 정의해야 함

이유는 컴파일러가 내 코드를 컴파일하기 전에 매크로 코드를 먼저 컴파일해서 실행해야 하기 때문입니다.


## 4단계: 절차적 매크로 심화 (Attribute & Function-like)

이번에는 함수 본문(Body)을 조작하고 매크로에 인자(Argument)를 전달하는 진짜 강력한 기능을 구현해 보겠습니다.

### 예제

구현할 매크로는 다음과 같은 기능을 가집니다.
```rust
#[log_time("DB_QUERY")] // 매크로에 "라벨"을 인자로 전달
fn intricate_calculation() {
    // 원래 로직 ...
}
```

이 매크로는 컴파일될 때, 함수 코드를 다음과 같이 **감싸서(Wrap)** 재작성합니다.

```rust
fn intricate_calculation() {
    let start = std::time::Instant::now(); // 1. 시작 시간 기록
    
    { 
        // 원래 로직이 여기 들어갑니다
    }

    // 2. 종료 후 경과 시간 출력 (매크로 인자로 받은 라벨 사용)
    println!("DB_QUERY took: {:?}", start.elapsed()); 
}
```

매크로 로직 구현해보겠습니다. 코드는 크게 두 부분으로 나뉩니다:
1. 속성 파싱 (`attr`): `("DB_QUERY")` 같은 매크로 인자를 읽습니다.
2. 함수 재조립 (`item`): 원래 함수 앞뒤에 타이머 코드를 붙입니다.

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, NestedMeta, Lit};

#[proc_macro_attribute]
pub fn log_time(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1. 매크로 인자 파싱 (예: #[log_time("MyLabel")])
    // AttributeArgs는 쉼표로 구분된 리터럴 목록을 파싱해줍니다.
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // 인자에서 라벨 텍스트 추출 (기본값 설정 로직)
    let label = if let Some(NestedMeta::Lit(Lit::Str(lit_str))) = args.first() {
        lit_str.value()
    } else {
        "Function".to_string() // 인자가 없으면 기본값
    };

    // 2. 적용할 함수 파싱
    let input_fn = parse_macro_input!(item as ItemFn);

    // 원래 함수의 구성요소 분리
    let fn_vis = &input_fn.vis;       // pub 같은 가시성
    let fn_sig = &input_fn.sig;       // fn 이름(인자) -> 리턴타입
    let fn_block = &input_fn.block;   // { 원래 코드 내용 }

    // 3. 코드 재조립 (quote! 사용)
    // 원래 블록(#fn_block)을 새로운 블록 안에 넣고, 앞뒤에 로직을 추가합니다.
    let output = quote! {
        #fn_vis #fn_sig {
            let start_time = std::time::Instant::now();
            
            // 원래 함수 내용 실행 및 리턴값 저장
            let result = (|| {
                #fn_block
            })();

            // 시간 측정 출력
            println!("[{}] took: {:?}", #label, start_time.elapsed());

            result
        }
    };

    output.into()
}
```

### 코드 상세 분석

#### 인자 파싱 (`AttributeArgs`)

`syn::AttributeArgs`는 `#[macro(arg1, arg2)]` 형태의 입력을 쉽게 파싱해주는 도우미 타입입니다. 여기서 `args.first()`를 통해 첫 번째 인자가 문자열 리터럴인지 확인하고 값을 꺼냈습니다.

#### 샌드위치 패턴 (Wrapping)

이것이 어트리뷰트 매크로의 핵심입니다.

- Before: `let start_time = ...`
- Center: `#fn_block` (원래 코드)
- After: `println!(...)`

주의: 원래 함수가 값을 반환할 수도 있으므로(예: `-> i32`), 원래 블록을 실행한 결과를 `let result`에 담아두었다가 마지막에 반환해야 합니다. 위 코드에서는 클로저 `(|| { #fn_block })()` 패턴을 사용하여 흐름을 제어했습니다.

## 5단계: `Derive` 매크로

이것은 `struct`나 `enum`의 정의만 보고, 그에 딸린 코드를 몽땅 자동으로 생성해주는 마법 같은 도구입니다.

이번 목표는 **"Builder 패턴 자동화"** 입니다. 복잡한 구조체를 만들 때 우리는 종종 Builder 패턴을 쓰지만, 이걸 손으로 짜는 건 정말 지루한 반복 작업(Boilerplate)입니다. 이걸 매크로 한 방으로 해결해 봅시다.

```rust
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields};

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
```

```rust
use my_macro_crate::Builder; // 크레이트 이름

#[derive(Builder, Debug)] // Debug는 출력을 위해 추가
pub struct Command {
    executable: String,
    args: Vec<String>,
    current_dir: String,
}

fn main() {
    // 1. 빌더 패턴 사용
    let command = Command::builder()
        .executable("cargo".to_string())
        .args(vec!["build".to_string(), "--release".to_string()])
        .current_dir(".".to_string())
        .build()
        .unwrap();

    println!("Command created: {:?}", command);
    
    // 2. 에러 케이스 (필드 누락)
    let bad_command = Command::builder()
        .executable("ls".to_string())
        .build(); // args, current_dir 누락
        
    println!("Error: {:?}", bad_command.err());
}
```

- `proc_macro_derive`: 구조체(`DeriveInput`)를 받아서 추가 코드(`impl`)를 생성합니다.
- `format_ident!`: "Struct이름" + "Builder" 처럼 새로운 식별자(이름)를 만들 때 사용합니다.
- 반복 (`#(...)*`): `quote!` 매크로의 강력한 기능으로, 필드 목록을 순회하며 코드를 생성합니다.
- 자동화: 이제 구조체 필드가 100개여도 코드를 한 줄도 수정할 필요가 없습니다. 매크로가 알아서 다 해줍니다.

## 6단계: 필드 속성 제어하기 (Field Attributes)

지금까지 만든 매크로는 모든 필드에 대해 **"필드명과 똑같은 이름"** 의 Setter 메서드를 만들었습니다. 하지만 실무에서는 이름이 마음에 안 들거나, 특별한 동작을 지시하고 싶을 때가 있죠.

이번 목표는 필드 위에 붙은 속성(Attribute)을 파싱하여 매크로의 동작을 제어하는 것입니다.

```rust
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Field, Meta, NestedMeta, Lit};

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
```
- `attr`: `#[builder(rename = "exe")]` 전체
- `path`: builder (속성 이름 확인)
- `parse_meta()`: 괄호 안의 구조를 분석 가능한 상태(Meta)로 변환
- `Meta::List`: `rename = "exe", other = "..."` 처럼 쉼표로 구분된 리스트
- `NestedMeta::Meta`: 리스트 안의 각 항목
- `Meta::NameValue`: `key = value` 형태 (`rename = "exe"`)
- `Lit::Str`: 실제 값 "exe"

## 7단계: 타입 분석 (Option과 Vec 내부 타입 꺼내기)

지금까지는 필드 타입이 무엇이든 그냥 `input: #ty` 형태로 받아왔습니다. 하지만 `Option<String>` 타입의 필드가 있을 때:
- 현재 상황: 사용자가 `.field(Some("hello".to_string()))` 라고 입력해야 함 (귀찮음 😩)
- 원하는 상황: 사용자가 `.field("hello".to_string())` 라고 입력하면 알아서 `Some`으로 감싸짐 (편안함 😎)
이를 위해서는 매크로가 "이 타입이 `Option`인가?" 를 판단하고, 만약 그렇다면 "그 안의 `T`는 무엇인가?" 를 추출해야 합니다.

### 이번 단계의 목표: 타입 껍질 까기 (unwrapping types)
우리는 `syn::Type`이라는 거대한 열거형(`Enum`)을 해부해야 합니다. `Option<String>`은 AST(추상 구문 트리) 내부에서 대략 다음과 같이 생겼습니다:

`Type` ➡ `Path` ➡ `Segments` ("Option") ➡ `Arguments` (`<String>`) ➡ `GenericArgument` ➡ `Type` ("String")

```rust
use syn::{Type, Path, PathArguments, GenericArgument};

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
// ... imports ...

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
    let mut builder_init = Vec::new();   // 빌더 초기화용
    let mut setters = Vec::new();        // Setter 메소드용
    let mut build_logic = Vec::new();    // Build 메소드 내부 로직용

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
```

- `syn::Type` 탐색: AST 구조는 깊고 복잡합니다. Path -> Segments -> Arguments -> GenericArgument 순서로 파고들어야 합니다.
- `Option` 처리 전략:
  - `Setter` 인자: `Option<T>`에서 `T`만 받도록 변경하여 사용자 경험(DX) 향상.
  - `Build` 로직: `Option` 필드는 값이 없어도 에러를 내지 않고 `None`을 그대로 전달 (`unwrap/ok_or` 불필요).
- 유연성: 이제 필드 타입에 따라 코드 생성 로직을 분기(Branching)하는 법을 익혔습니다.

## 8단계: `Vec<T>` 필드에 대한 처리

보통 Vec 필드는 .args(vec![a, b]) 처럼 한 번에 넣기도 하지만, .arg(a).arg(b) 처럼 하나씩 추가하고 싶을 때가 많습니다.

Vec<String> 타입의 필드가 있을 때, 사용자는 두 가지 방식을 원할 수 있습니다.

- Bulk: `.args(vec!["-a", "-b"])` (기본 동작)
- Incremental: `.arg("-a").arg("-b")` (우리가 만들 기능)

### 핵심 요약
1. 조건부 코드 생성: Option이냐, Vec이냐, 일반 타입이냐에 따라 필드 정의, 초기화, Setter, Build 로직 4가지가 모두 다르게 생성되어야 합니다.
2. each 패턴: 컬렉션(Vec) 타입의 경우, 전체를 교체하는 Setter보다 하나씩 추가하는(Append) Setter가 더 유용할 때가 많습니다. 이를 속성으로 제어했습니다.
3. 초기화 전략:
  - 일반/Option 필드: None으로 초기화.
  - each 속성 Vec 필드: Vec::new() (빈 벡터)로 초기화하여 바로 push 가능하게 함.

## 9단계: 디버깅(Debugging), 위생(Hygiene), 그리고 우아한 에러 처리

### 디버깅: 도대체 내 코드가 어떻게 변한 거야?
매크로는 컴파일 타임에 코드를 생성합니다. `println!`을 써도 런타임 콘솔에는 나오지 않죠. 매크로가 확장된(Expanded) 최종 코드를 눈으로 확인해야 합니다.

#### 필수 도구: `cargo expand`

이것은 선택이 아니라 필수입니다.

- 설치: `cargo install cargo-expand`
- 사용: 터미널에서 `cargo expand` 입력

이 명령어를 치면 매크로가 전부 풀려서 실제 컴파일러가 보게 될 Rust 코드가 출력됩니다.

Tip: 코드가 너무 길면 `cargo expand --bin main` 처럼 특정 타겟만 지정하거나, `cargo expand | code` - (VS Code 사용 시)로 에디터에서 바로 볼 수 있습니다.

### 위생(Hygiene): 이름 충돌 방지

**매크로 위생(Macro Hygiene)** 이란 매크로 내부의 변수나 임포트가 사용자의 코드와 충돌하지 않도록 격리하는 것을 말합니다.

#### ❌ 나쁜 예 (충돌 발생)
만약 우리가 만든 `log_time` 매크로가 내부적으로 `result`라는 변수명을 쓴다고 가정해 봅시다.

```rust
// 매크로 내부 (우리가 작성한 코드)
let result = ...; // 매크로가 생성한 변수

// 사용자 코드 (매크로를 쓰는 사람)
#[log_time]
fn user_fn() {
    let result = 100; // 사용자가 이미 result를 쓰고 있다면?
    // 💥 쾅! 변수 재정의(Shadowing)나 타입 불일치 에러 발생!
}
```

#### ✅ 해결책 1: quote!의 마법 (Local Hygiene)
다행히 `quote!` 매크로는 기본적으로 지역 변수 위생을 챙겨줍니다. `quote!` 내에서 생성된 변수는 컴파일러가 내부적으로 고유한 식별자를 부여하여 사용자의 코드와 섞이지 않게 해줍니다.

#### ✅ 해결책 2: 전체 경로(Fully Qualified Path) 사용 (Global Hygiene)
가장 흔한 실수는 외부 타입/함수를 쓸 때 발생합니다.

```rust
// ❌ 위험한 코드
let start = Instant::now(); // 사용자가 'use std::time::Instant;'를 안 했다면 에러!

// ✅ 안전한 코드 (절대 경로)
let start = ::std::time::Instant::now();
```
규칙: 매크로가 생성하는 코드 내의 모든 타입과 함수는 반드시 `::std::...` 처럼 루트(`::`)부터 시작하는 전체 경로를 써야 합니다. 사용자의 `use` 문에 의존하지 마세요.

### 우아한 에러 처리: `compile_error!`

만약 사용자가 매크로를 잘못 썼다면? 패닉(`panic!`)을 일으켜 컴파일러를 터뜨리는 대신, 예쁜 빨간 줄(에러 메시지)을 띄워줘야 합니다.

`syn`과 `proc_macro`는 이를 위해 `compile_error!`를 제공합니다.

지난번 `log_time` 매크로를 수정하여, "인자가 문자열이 아니면 에러를 내뿜는" 기능을 넣어보겠습니다.

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, NestedMeta, Lit};

#[proc_macro_attribute]
pub fn log_time(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input_fn = parse_macro_input!(item as ItemFn);

    let label = if let Some(arg) = args.first() {
        if let NestedMeta::Lit(Lit::Str(lit_str)) = arg {
            lit_str.value()
        } else {
            // 🚨 에러 처리: 문자열이 아닌 경우
            // syn::Error::new_spanned는 에러가 발생한 코드 위치(span)를 정확히 가리킵니다.
            return syn::Error::new_spanned(
                arg, 
                "인자는 반드시 문자열 리터럴이어야 합니다 (예: \"Label\")"
            )
            .to_compile_error() // 컴파일 에러 토큰으로 변환
            .into();
        }
    } else {
        "Function".to_string()
    };

    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;

    // ✅ 위생(Hygiene) 적용: ::std::time::Instant (절대 경로)
    let output = quote! {
        #fn_vis #fn_sig {
            let start_time = ::std::time::Instant::now(); 
            
            let result = (|| {
                #fn_block
            })();

            println!("[{}] took: {:?}", #label, start_time.elapsed());

            result
        }
    };

    output.into()
}
```

- `syn::Error::new_spanned`는 에러가 발생한 소스 코드의 위치(Span) 정보를 담아서 에러를 생성합니다. 이것이 있어야 빨간 밑줄이 정확한 위치에 그어집니다.

### 매크로 조언 요약

#### ✅ 1. 매크로는 최후의 수단이다 (Don't Overuse)
매크로는 강력하지만 코드를 읽기 어렵게 만들고, 컴파일 시간을 늘립니다. 함수(Function), 트레이트(Trait), 제네릭(Generic)으로 해결할 수 있다면 그것들을 먼저 사용하세요. 매크로는 **"지루한 반복 코드를 줄일 때"** 만 사용하세요.

#### ✅ 2. cargo expand는 친구다
매크로가 이상하게 동작하면 고민하지 말고 무조건 `cargo expand`를 실행하세요. 매크로가 생성한 실제 코드를 눈으로 확인하는 것이 디버깅의 지름길입니다.

#### ✅ 3. 위생(Hygiene)을 지켜라
매크로가 생성하는 코드 안에서는 항상 `::std::option::Option` 처럼 절대 경로를 사용하세요. 사용자가 `use std::option::Option as Opt;` 처럼 이름을 바꿔서 쓰고 있을지도 모릅니다.

#### ✅ 4. 테스트는 trybuild로
우리가 방금 한 것처럼 "에러가 잘 나는지" 테스트하는 것도 중요합니다. `trybuild라`는 크레이트를 사용하면, 컴파일 에러 메시지 자체를 테스트 케이스로 관리할 수 있습니다. (오픈소스 라이브러리를 만들 생각이라면 필수입니다.)