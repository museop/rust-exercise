#[derive(Debug)]
struct MyString {
    data: String,
}

impl MyString {
    // 제네릭 매개변수 T는 Into<String> 트레잇을 구현해야 함
    // Into<String>: T 타입을 String으로 변환하는 기능 제공
    // &str 일 경우, 새로운 String이 생성됨 (할당 발생)
    // String 일 경우, 소유권이 이동됨
    fn new<T: Into<String>>(s: T) -> MyString {
        Self { data: s.into() }
    }

    fn print(&self) {
        println!("{}", self.data);
    }
}

fn main() {
    // 1) Owned String -> Into<String> : move (no allocation)
    let s = String::from("hello");
    let t: MyString = MyString::new(s); // s가 이동(moved)됨
    // println!("{}", s); // ✖ 컴파일 오류: use of moved value `s`
    t.print(); // ok

    // 2) &str -> Into<String> : 새로운 String 생성(할당)
    let slice: &str = "hello";
    let u: MyString = MyString::new(slice); // slice의 내용을 복사해서 String 생성
    println!("{}", slice); // ok, slice는 여전히 사용 가능
    u.print();

    // 3) &String -> &str 슬라이스로 변환 후 into (안전한 방법)
    let owned = String::from("world");
    let v: MyString = MyString::new(&owned[..]); // &owned[..]는 &str
    println!("{}", owned); // ok, owned는 여전히 유효
    v.print();
}
