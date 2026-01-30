use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    // 다중 생산자, 다중 소비자를 위한 채널을 생성합니다.
    // mpsc는 다중 생산자, 단일 소비자 채널이지만,
    let (tx, rx) = mpsc::channel();

    // 수신자(rx)를 Arc<Mutex<T>>로 감싸서 여러 소비자 스레드가 공유할 수 있도록 합니다.
    let rx = Arc::new(Mutex::new(rx));

    // 소비자(워커) 스레드들을 담을 벡터를 생성합니다.
    let mut consumer_handles = vec![];
    const NUM_CONSUMERS: u32 = 2; // 소비자 스레드 개수

    for i in 0..NUM_CONSUMERS {
        // 각 소비자 스레드를 위해 공유 수신자의 Arc를 복제합니다.
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            loop {
                // 뮤텍스 락을 획득하고 채널에서 메시지를 수신합니다.
                // recv()는 메시지가 있을 때까지 블로킹됩니다.
                // 채널이 닫히면 Err를 반환하고 루프가 종료됩니다.
                let message = rx_clone.lock().unwrap().recv();
                match message {
                    Ok(msg) => {
                        println!("워커 {}가 메시지를 받았습니다: {}", i, msg);
                        // 작업을 처리하는 것을 시뮬레이션하기 위해 잠시 대기합니다.
                        thread::sleep(Duration::from_millis(500));
                    }
                    Err(_) => {
                        // 송신자들이 모두 사라지면 채널이 닫히고, recv()는 에러를 반환합니다.
                        println!("워커 {}가 모든 작업을 마쳤습니다.", i);
                        break;
                    }
                }
            }
        });
        consumer_handles.push(handle);
    }

    // 생산자 스레드들을 담을 벡터를 생성합니다.
    let mut producer_handles = vec![];
    const NUM_PRODUCERS: u32 = 4;
    const MESSAGES_PER_PRODUCER: u32 = 5;

    for i in 0..NUM_PRODUCERS {
        // 각 생산자 스레드를 위해 송신자(tx)를 복제합니다.
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            for j in 0..MESSAGES_PER_PRODUCER {
                let message = format!("생산자 {}가 보낸 메시지 {}", i, j);
                println!("생산자 {}가 메시지 전송: {}", i, &message);
                // 복제된 송신자를 통해 채널에 메시지를 보냅니다.
                tx_clone.send(message).unwrap();
                // 작업을 생성하는 것을 시뮬레이션하기 위해 잠시 대기합니다.
                thread::sleep(Duration::from_millis(100));
            }
        });
        producer_handles.push(handle);
    }

    // 채널은 모든 송신자(tx)가 드롭되어야 닫힙니다.
    // 메인 스레드의 송신자를 드롭(drop)합니다.
    // 이렇게 해야 수신자(rx)가 모든 메시지를 받은 후 대기를 멈출 수 있습니다.
    drop(tx);

    // 모든 생산자 스레드가 끝날 때까지 기다립니다.
    for handle in producer_handles {
        handle.join().unwrap();
    }
    println!("모든 생산자가 작업을 마쳤습니다.");

    // 모든 소비자 스레드가 끝날 때까지 기다립니다.
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("모든 작업이 완료되었습니다!");
}
