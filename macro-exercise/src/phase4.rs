use my_macro_attribute::log_time; // 크레이트 이름에 맞게 수정
use std::thread;
use std::time::Duration;

#[log_time("HeavyTask")]
fn heavy_computation() -> i32 {
    println!("Doing hard work...");
    thread::sleep(Duration::from_millis(500));
    42
}

#[log_time] // 인자 없이 사용 (기본값 "Function" 출력)
fn quick_task() {
    println!("Quick work!");
}

fn main() {
    let res = heavy_computation();
    println!("Result: {}", res);

    quick_task();
}
