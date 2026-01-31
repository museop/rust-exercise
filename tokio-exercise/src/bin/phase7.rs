use futures::SinkExt; // 데이터를 보낼 때 필요 (.send)
use futures::StreamExt;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LinesCodec}; // 데이터를 받을 때 필요 (.next)

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("🚀 Codec 에코 서버 시작 (127.0.0.1:8080)");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("✨ 접속: {}", addr);

        tokio::spawn(async move {
            // 1. Codec 준비
            // LinesCodec은 줄바꿈(\n)을 기준으로 데이터를 자르는 규칙을 가집니다.
            let codec = LinesCodec::new();

            // 2. Framed 생성 (Socket + Codec)
            // 이제 'socket'은 단순한 바이트 파이프가 아니라,
            // 'String 메시지'를 주고받는 객체(frame)로 변신합니다.
            let mut frame = Framed::new(socket, codec);

            // 3. 데이터 수신 (Stream)
            // loop { socket.read(...) } 대신, while let ... frame.next()를 씁니다.
            // .next()는 완벽한 한 줄이 완성될 때까지 내부 버퍼에 데이터를 모으며 기다립니다.
            while let Some(result) = frame.next().await {
                match result {
                    Ok(line) => {
                        // 'line'은 이미 Vec<u8>이 아니라 String입니다!
                        println!("수신: {}", line);

                        // 4. 데이터 전송 (Sink)
                        // .send()에 String을 넣으면 Codec이 알아서 바이트로 변환하고 줄바꿈을 붙여서 보냅니다.
                        let response = format!("Echo: {}", line);
                        if let Err(e) = frame.send(response).await {
                            eprintln!("전송 에러: {}", e);
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("통신 에러: {}", e);
                        return;
                    }
                }
            }
            println!("👋 연결 종료: {}", addr);
        });
    }
}

// 실무에서는 채팅(텍스트)만 하는 게 아닙니다. 파일 전송이나 게임 서버라면 "길이 기반 프로토콜(Length-Prefixed)"을 많이 씁니다. (예: 앞 4바이트는 길이, 뒤에는 내용)
// 이때는 LengthDelimitedCodec을 쓰면 됩니다.
// use tokio_util::codec::{Framed, LengthDelimitedCodec};
// // LinesCodec 대신 이걸로만 바꾸면 됩니다!
// let codec = LengthDelimitedCodec::new();
// let mut frame = Framed::new(socket, codec);
// // 이제 frame.next()는 "Bytes" 덩어리(BytesMut)를 반환합니다.
// // 내용은 바이너리 이미지 데이터일 수도, Protobuf일 수도 있습니다.
