use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener; // 표준 라이브러리의 Mutex 사용

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("📊 카운터 서버 시작 (127.0.0.1:8080)");

    // 1. 공유 상태 생성 (Arc<Mutex<T>>)
    // 숫자를 Mutex로 감싸고, 다시 Arc로 감싸서 여러 태스크로 보낼 준비를 합니다.
    let global_count = Arc::new(Mutex::new(0));

    loop {
        let (mut socket, _) = listener.accept().await?;

        // 2. 참조 카운트 증가 (Clone)
        // Arc::clone은 데이터를 복사하는 게 아니라, "나도 이걸 가리킬래"라고 포인터만 하나 더 만드는 것입니다.
        // 이 clone된 포인터를 태스크로 보냅니다.
        let count_handle = Arc::clone(&global_count);

        tokio::spawn(async move {
            // 3. 락 획득 및 데이터 수정
            // 데이터를 준비하는 부분 (락이 필요한 구간)을 별도 블록으로 감쌉니다.
            let msg = {
                let mut num = count_handle.lock().unwrap();
                *num += 1; // 숫자 증가
                println!("현재 방문자 수: {}", *num);

                // 보낼 메시지 준비
                format!("당신은 {}번째 방문자입니다!\n", *num)
            }; // 여기서 락이 해제됩니다.

            // 4. I/O 작업 (락 없이 수행)
            if let Err(e) = socket.write_all(msg.as_bytes()).await {
                eprintln!("데이터 전송 오류: {}", e);
            }
        });
    }
}

// 만약 로직이 복잡해서 락을 잡은 채로 꼭 .await를 해야 한다면(예: DB 트랜잭션 등),
// Tokio 버전의 tokio::sync::Mutex 써야 합니다.
// 1. import 변경
// use tokio::sync::Mutex; // std::sync::Mutex 대신 이걸 씁니다.
// use std::sync::Arc;
// ... (중략) ...
//     let global_count = Arc::new(Mutex::new(0));
//     loop {
//         // ...
//         tokio::spawn(async move {
//             // 2. lock() 뒤에 .await가 붙습니다! (비동기 락)
//             let mut num = count_handle.lock().await;
//             *num += 1;
//             let msg = format!("당신은 {}번째 방문자입니다!\n", *num);
//             // 3. 락을 쥔 채로 I/O를 해도 괜찮습니다. (Tokio Mutex는 Send가 구현되어 있음)
//             // 하지만 성능상으로는 여전히 락을 빨리 푸는 게 좋습니다.
//             if let Err(e) = socket.write_all(msg.as_bytes()).await {
//                  eprintln!("전송 에러: {}", e);
//             }
//         });
//     }
