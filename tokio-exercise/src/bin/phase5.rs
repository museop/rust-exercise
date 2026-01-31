use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // 1. 채널 생성 (버퍼 크기 32)
    // tx: 송신자 (Transmitter) - 복제 가능
    // rx: 수신자 (Receiver) - 복제 불가능 (단일 소유권)
    // 버퍼 크기: 채널이 꽉 차면 송신자들은 자리가 날 때까지 대기(.await)합니다. (Backpressure 기능)
    let (tx, mut rx) = mpsc::channel(32);

    // 2. 여러 개의 송신 태스크 생성 (Producer)
    for i in 1..=5 {
        // 각 태스크마다 tx를 하나씩 복재해서 가져갑니다.
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            println!("🤖 센서 {} 가동 시작", i);

            // 데이터 전송
            let msg = format!("센서 {}의 측정값: {}", i, i * 10);

            // send().await: 채널이 꽉 차 있으면 기다림
            if let Err(e) = tx_clone.send(msg).await {
                eprintln!("센서 {} 데이터 전송 실패: {}", i, e);
            } else {
                println!("✅ 센서 {} 데이터 전송 완료", i);
            }
        });
    }

    // 3. 중요! 메인 함수가 가진 원본 'tx'는 여기서 버려야 합니다.
    // 안 버리면? 수신자(rx)가 모든 송신자가 종료되었다고 인식하지 못합니다.
    drop(tx);

    // 4. 수신 루프 (Consumer)
    println!("📡 관제 센터 수신 대기 중...");

    // rx.recv()는 모든 tx가 닫힐 때까지 데이터를 기다립니다.
    // 모든 tx가 drop되면 None을 반환하고 반복문이 종료됩니다.
    while let Some(received) = rx.recv().await {
        println!("📥 관제 센터 수신: {}", received);
    }

    println!("🏁 모든 센서 데이터 수신 완료, 관제 센터 종료");
}
