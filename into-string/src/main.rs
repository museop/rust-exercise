fn main() {
    // 1) Owned String -> Into<String> : move (no allocation)
    let s = String::from("hello");
    let t: String = s.into(); // s가 이동(moved)됨
    // println!("{}", s); // ✖ 컴파일 오류: use of moved value `s`
    println!("{}", t); // ok

    // 2) &str -> Into<String> : 새로운 String 생성(할당)
    let slice: &str = "hello";
    let u: String = slice.into(); // slice의 내용을 복사해서 String 생성
    println!("{}", slice); // ok, slice는 여전히 사용 가능
    println!("{}", u);

    // 3) &String -> &str 슬라이스로 변환 후 into (안전한 방법)
    let owned = String::from("world");
    let v: String = (&owned[..]).into(); // &owned[..]는 &str
    println!("{}", owned); // ok, owned는 여전히 유효
    println!("{}", v);
}
