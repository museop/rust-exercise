use std::{borrow::Cow, ffi::OsStr};

// Unix 계열(Linux, macOS)에서 바이트로 OsStr을 만들기 위한 트레이트
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

fn main() {
    println!("--- 1. 정상적인  UTF-8 문자열 ---");
    let valid_source = "Hello, 무섭!";
    let valid_os_str = OsStr::new(valid_source);

    // 변환 시도
    let cow_valid = valid_os_str.to_string_lossy();

    // Cow 타입을 통해 빌려온 것인지(Borrowed) 소유한 것인지(Owned) 확인
    match cow_valid {
        Cow::Borrowed(s) => println!("결과: Borrowed (복사 안함) -> {}", s),
        // ref s를 사용하여 소유권을 가져오지 않고 '참조'만 함
        Cow::Owned(ref s) => println!("결과: Owned (새로 생성됨) -> {}", s),
    }

    // 최종적으로 String으로 만들기
    let final_string: String = cow_valid.into_owned();
    println!("최종 String: {}", final_string);

    println!("\n--- 2. 유효하지 않은 UTF-8 케이스 (깨진 문자) ---");

    #[cfg(unix)]
    let invalid_bytes = b"Hello \xFF World";
    #[cfg(not(unix))]
    let invalid_bytes = b"Hello World";

    #[cfg(unix)]
    let invalid_os_str = OsStr::from_bytes(invalid_bytes);
    #[cfg(not(unix))]
    let invalid_os_str = OsStr::new("Hello World (Non-Unix Fallback)");

    let cow_invalid = invalid_os_str.to_string_lossy();

    match cow_invalid {
        Cow::Borrowed(s) => println!("결과: Borrowed -> '{}'", s),
        // [수정됨] 역시 ref를 사용하여 빌려옵니다.
        Cow::Owned(ref s) => {
            println!("결과: Owned (깨진 문자 대체 및 할당 발생!)");
            println!("변환된 문자열: '{}'", s);
        }
    }

    let final_lossy_string: String = cow_invalid.into_owned();
    println!("최종 String: {}", final_lossy_string);
}
