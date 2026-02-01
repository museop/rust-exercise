use my_macro_derive_option_fields::Builder;

#[derive(Builder, Debug)]
pub struct Config {
    // Option 타입 필드
    log_level: Option<String>,

    // 일반 필수 필드
    api_key: String,
}

fn main() {
    let config = Config::builder()
        .api_key("secret123".to_string())
        // 중요: Some("Info".to_string()) 이 아니라 그냥 String을 넣습니다!
        .log_level("Info".to_string())
        .build()
        .unwrap();

    println!("Config: {:?}", config);

    // log_level을 설정 안 해도 빌드 성공 (Option이니까)
    let min_config = Config::builder()
        .api_key("key_only".to_string())
        .build()
        .unwrap();

    println!("Min Config: {:?}", min_config);
}
