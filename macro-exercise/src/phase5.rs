use my_macro_derive::Builder; // 크레이트 이름

#[derive(Builder, Debug)] // Debug는 출력을 위해 추가
pub struct Command {
    executable: String,
    args: Vec<String>,
    current_dir: String,
}

impl Command {
    fn print_fields(&self) {
        println!(
            "Command: current_dir={:?} executable={:?} args={:?}",
            self.current_dir, self.executable, self.args
        )
    }
}

fn main() {
    // 1. 빌더 패턴 사용
    let command = Command::builder()
        .executable("cargo".to_string())
        .args(vec!["build".to_string(), "--release".to_string()])
        .current_dir(".".to_string())
        .build()
        .unwrap();

    println!("Command created: {:?}", command);
    command.print_fields();

    // 2. 에러 케이스 (필드 누락)
    let bad_command = Command::builder().executable("ls".to_string()).build(); // args, current_dir 누락

    println!("Error: {:?}", bad_command.err());
}
