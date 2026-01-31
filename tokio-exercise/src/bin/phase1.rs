use tokio::time::{sleep, Duration};

async fn say_hello() {
    println!("hello");

    // 비동기적인 대기 (스레드를 멈추지 않고 1초 쉼)
    sleep(Duration::from_secs(1)).await;

    println!("world");
}

// #[tokio::main] 매크로는 비동기 런타임을 시작시키는 매크로입니다.
// 이 매크로가 없으면 main 함수에는 async 키워드를 붙일 수 없습니다.
#[tokio::main]
async fn main() {
    println!("1. 함수 호출 시작");

    let future = say_hello();

    println!("2. 함수 호출 했으나 실행은 안 됨 (이 메지시가 hello보다 먼너 나옴)");

    // 실제로 Future를 실행하려면 .await가 필요
    future.await;

    println!("3. 함수 호출 완료");
}
