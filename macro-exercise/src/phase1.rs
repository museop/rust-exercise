// 1. 매크로 정의
macro_rules! say_hello {
    // () => {} : 인자가 없을 때 이 코드로 바꿔라!
    () => {
        println!("Hello, Macro World!");
    };
}

fn main() {
    // 2. 매크로 호출
    say_hello!();

    // 컴파일러는 위 줄을 아래처럼 바꿉니다.
    // println!("Hello, Macro World!");
}
