use std::{
    ffi::{CString, OsStr},
    path::{Path, PathBuf},
};

fn main() {
    println!("--- 1. String & &str (표준 문자열) ---");

    // 1-1. String (소유): 힙에 저장되는 가변 문자열
    let mut base_name = String::from("my_report");
    base_name.push_str(".txt"); // 문자열 추가

    // 1-2. &str (참조): String을 빌려서 사용
    // 함수에 넘길 때는 &String보다 &str이 훨씬 유연합니다.
    print_utf8_into(&base_name);

    println!("--- 2. PathBuf & Path (파일 경로) ---");

    // 2-1. PathBuf (소유): 힙에 저장되는 가변 파일 경로
    let mut path = PathBuf::from("/Users/museop");
    path.push(&base_name); // String을 경로에 추가
    path.set_extension("md"); // 확장자 설정

    // 2-2. Path (참조)
    let path_slice: &Path = path.as_path();
    println!("파일 경로: {:?}", path_slice);

    println!("--- 3. OsStr (운영체제 호환) ---");

    // 3-1. Path에서 파일명만 추출 (OsStr 리턴)
    // 파일명에 UTF-8이 아닌 바이트가 섞여 있을 수 있으므로 OsStr로 반환합니다.
    if let Some(os_str) = path.file_name() {
        print_os_into(os_str);

        // 3-2. OsStr -> String 변환 (손실 가능성 있음)
        let safe_string = os_str.to_string_lossy();
        println!("안전하게 변환된 문자열: {}", safe_string);
    }

    println!("--- 4. CString (C 언어 연동) ---");

    // 4-1. Rust String -> CString 변환 (Null Byte 추가)
    // 중간에 '\0'이 있으면 에러가 나므로 Result를 처리해야 합니다.
    let c_string = CString::new(base_name).expect("CString 생성 실패");

    // 4-2. C 함수에 포인터 전달 (unsafe)
    // as_ptr()은 *const c_char 타입을 반환합니다.
    unsafe {
        dummy_c_function(c_string.as_ptr());
    }
}

// --- Helper Functions ---

// 1. 표준 문자열을 받는 함수 (가장 일반적)
fn print_utf8_into(s: &str) {
    println!("문자열 길이(Bytes): {}, 내용: {}", s.len(), s);
}
// 2. OS 문자열을 받는 함수
fn print_os_into(os_s: &OsStr) {
    println!("OS 레벨 문자열 길이: {:?}", os_s.len());
}
// 3. 가상의 C 라이브러리 함수
unsafe fn dummy_c_function(ptr: *const i8) {
    println!("C 함수가 주소 {:?} 에서 데이터를 받았습니다.", ptr);
}
