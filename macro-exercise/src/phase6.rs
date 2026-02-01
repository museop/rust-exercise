use my_macro_derive_attributes::Builder;

#[derive(Builder, Debug)]
pub struct Command {
    #[builder(rename = "exe")] // 이제 executable() 대신 exe()를 써야 합니다.
    executable: String,

    args: Vec<String>,
}

fn main() {
    let cmd = Command::builder()
        .exe("cargo".to_string()) // executable(...) 이라고 쓰면 컴파일 에러!
        .args(vec!["run".to_string()])
        .build()
        .unwrap();

    println!("Success: {:?}", cmd);
}
