use std::fs;

#[derive(Debug, thiserror::Error)]
enum ConfigError {
    // #[error("...")]는 Display 트레이트를 구현 (오류 메시지)
    #[error("Failed to read file: {0}")]
    FileReadError(String),

    // #[from]은 std::io::Error가 발생하면
    // 자동으로 ConfigError::IoError로 변환해줌
    #[error("I/O error occurred: {0}")]
    IoError(#[from] std::io::Error),

    // serde_json::Error가 발생하면
    // 자동으로 ConfigError::JsonError로 변환해줌
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
}

fn get_config_content() -> Result<String, ConfigError> {
    let content = fs::read_to_string("config.txt")?;

    let parsed: serde_json::Value = serde_json::from_str(&content)?;

    Ok(parsed.to_string())
}

fn main() {
    println!("{:?}", ConfigError::FileReadError("config.txt".to_string()));

    match get_config_content() {
        Ok(config) => println!("Config content: {}", config),
        Err(e) => match e {
            ConfigError::FileReadError(filename) => {
                eprintln!("Could not read the file: {}", filename);
            }
            ConfigError::IoError(io_err) => {
                eprintln!("I/O error occurred: {}", io_err);
            }
            ConfigError::JsonError(json_err) => {
                eprintln!("Failed to parse JSON: {}", json_err);
            }
        },
    }
}
