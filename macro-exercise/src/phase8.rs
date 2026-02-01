use my_macro_derive_vec_fields::Builder;

#[derive(Builder, Debug)]
pub struct Command {
    executable: String,

    // each를 사용: args(vec) 대신 arg(item) 사용 가능
    #[builder(each = "arg")]
    args: Vec<String>,

    // each를 사용: env(key, val) 대신 env(str) 사용
    #[builder(each = "env")]
    envs: Vec<String>,

    // 일반 Option
    current_dir: Option<String>,
}

fn main() {
    let cmd = Command::builder()
        .executable("cargo".to_string())
        .arg("build".to_string()) // 하나씩 추가!
        .arg("--release".to_string()) // 체이닝!
        .env("RUST_LOG=info".to_string())
        .build()
        .unwrap();

    println!("Command: {:?}", cmd);
}
