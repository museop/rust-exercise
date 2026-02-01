use hello_macro::HelloMacro; // 매크로 가져오기 

// Trait 정의 (매크로가 구현할 대상)
trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Dog;

fn main() {
    Pancakes::hello_macro();
    Dog::hello_macro();
}
