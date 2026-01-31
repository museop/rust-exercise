use tokio::time::{Duration, sleep};

async fn heavy_task() -> String {
    // 2초가 걸리는 무거운 작업
    println!("🔄 무거운 작업 시작...");
    sleep(Duration::from_secs(2)).await;

    "작업 완료!".to_string()
}

#[tokio::main]
async fn main() {
    println!("⏱️ 경쟁 시작!");

    let task_future = heavy_task();
    let timeout_future = sleep(Duration::from_secs(1));

    // select! 매크로를 사용하여 둘 중 먼저 완료되는 작업을 기다립니다.
    tokio::select! {
        // 첫 번째 가지: 작업이 성공했을 때
        result = task_future => {
            println!("✅ 무거운 작업이 먼저 완료되었습니다: {}", result);
        }
        // 두 번째 가지: 타임아웃이 발생했을 때
        _ = timeout_future => {
            println!("⏰ 시간 초과! 무거운 작업이 너무 오래 걸립니다.");
        }
    }

    loop {
        tokio::select! {
            // 1. 자동 종료를 위한 타임아웃
            _ = sleep(Duration::from_secs(5)) => {
                println!("⏳ 전체 타임아웃! 프로그램을 종료합니다.");
                break;
            }

            // 2. 종료 신호 감지 (tokio::signal 사용)
            // 윈도우/맥/리눅스 모두에서 Ctrl+C 신호를 감지합니다.
            _ = tokio::signal::ctrl_c() => {
                println!("🛑 종료 신호(Ctrl+C) 감지! 프로그램을 종료");
                break;
            }
        }
    }

    println!("🏁 프로그램 종료");
}
