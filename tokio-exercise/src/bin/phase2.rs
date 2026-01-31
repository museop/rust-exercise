use tokio::time::{Duration, sleep};

async fn heavy_computation(name: String, seconds: u64) -> String {
    println!("[{}] 작업 시작 ({}초 소요)", name, seconds);
    sleep(Duration::from_secs(seconds)).await;
    println!("[{}] 작업 완료", name);
    format!("{}의 결과물", name)
}

#[tokio::main]
async fn main() {
    println!("=== 프로그램 시작 ===");
    let start = std::time::Instant::now();

    // 1. spawn으로 작업 2개를 동시에 실행시킵니다.
    // spawn은 'JoinHandle'이라는 Future를 즉시 반환합니다. (작업의 결과를 나중에 받을 수 있는 핸들)
    // 이때 소유권 전송을 위해 async move {} 블록을 자주 사용합니다.
    let handle1 = tokio::spawn(async move { heavy_computation("작업 1".to_string(), 2).await });
    let handle2 = tokio::spawn(async move { heavy_computation("작업 2".to_string(), 1).await });

    println!("메인 함수는 작업을 던져두고 다른 일을 할 수 있습니다.");

    // 2. 작업이 끝날 때까지 기다리고 결과 값을 받습니다. (Join)
    // .await를 하지 않으면 메인 함수가 종료되면서 작업들이 강제 취소될 수 있습니다.
    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();

    println!("결과 취합: {}, {}", result1, result2);

    // 총 소요 시간 확인
    println!("총 소요 시간: {:.2?} (약 2초 이어야 함)", start.elapsed());
}
