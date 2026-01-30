use std::borrow::Cow;

// fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
// 입력 input이 살아있는 동안('a), 리턴되는 Cow도 동일한 수명을 갖도록 지정
// '_는 컴파일러가 수명을 추론하도록 하는 방법
fn remove_spaces(input: &str) -> Cow<'_, str> {
    if input.contains(' ') {
        // If there are spaces, create a new string without spaces
        Cow::Owned(input.replace(' ', ""))
    } else {
        // If there are no spaces, return a borrowed reference
        Cow::Borrowed(input)
    }
}

/*
// Cow<'a, str>의 실제 동작 방식
pub enum Cow<'a, str> {
    // 1. 빌려온 경우: &str을 담습니다.
    Borrowed(&'a str),

    // 2. 소유한 경우: str의 소유 버전인 String을 담습니다.
    Owned(String),
}
 */

fn main() {
    let s1 = "Hello_World"; // No spaces
    let result1 = remove_spaces(s1);

    match result1 {
        Cow::Borrowed(borrowed) => println!("s1: borrowed: {} (efficient)", borrowed),
        Cow::Owned(owned) => println!("s1: owned: {} (new)", owned),
    }

    let s2 = "Hello World"; // Contains spaces
    let result2 = remove_spaces(s2);

    match result2 {
        Cow::Borrowed(borrowed) => println!("s2: borrowed: {} (efficient)", borrowed),
        Cow::Owned(owned) => println!("s2: owned: {} (new)", owned),
    }
}
