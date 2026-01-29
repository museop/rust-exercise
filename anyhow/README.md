# `anyhow` 라이브러리(crate)

anyhow는 Rust의 오류 처리를 매우 간편하게 만들어주는 인기 있는 라이브러리(crate)입니다.

특히 애플리케이션(예: CLI 도구, 웹 서버)을 개발할 때, 다양한 종류의 오류를 일일이 변환하거나 감싸는 상용구(boilerplate) 코드 없이도 오류를 쉽게 전파하고, 유용한 '컨텍스트(문맥)'를 추가할 수 있도록 도와줍니다.

## anyhow를 사용하는 이유
Rust의 표준 오류 처리 `Result<T, E>`는 매우 강력하지만, `E`(오류 타입)가 구체적이어야 합니다. 애플리케이션에서는 `std::io::Error`, `serde_json::Error`, `reqwest::Error` 등 수많은 다른 종류의 오류가 발생할 수 있습니다.

`anyhow`가 없었다면, 이 모든 오류를 처리하기 위해 복잡한 `enum`을 정의하거나 `Box<dyn std::error::Error>`를 사용하고 `.map_err()`로 수동 변환해야 했습니다.

anyhow는 이 과정을 극도로 단순화합니다.

1. 간편한 오류 타입: `anyhow::Result`

-  anyhow는 `anyhow::Result<T>`라는 타입을 제공합니다. 이는 사실 `std::result::Result<T, anyhow::Error>`의 별칭입니다.
- `anyhow::Error`는 동적 오류 타입으로, Rust의 표준 `std::error::Error` 트레이트(trait)를 구현하는 모든 오류를 담을 수 있는 '만능' 오류 컨테이너입니다.

2. 마법 같은 `?` 연산자 호환
- `anyhow::Result`를 반환하는 함수 내에서는, `std::io::Error`를 반환하는 함수든 `serde_json::Error`를 반환하는 함수든 상관없이 `?` 연산자를 바로 사용할 수 있습니다. anyhow가 알아서 `anyhow::Error`로 변환해줍니다.

3. 풍부한 컨텍스트(Context) 추가
- 오류가 발생했을 때 "File not found"라는 메시지만으로는 부족할 때가 많습니다. "어떤 파일을", "무슨 작업을 하다가" 찾지 못했는지 알려주는 것이 디버깅에 훨씬 유용합니다.
- anyhow는 Context 트레이트를 제공하여 `.context()` 메서드로 간단하게 문맥을 추가할 수 있게 해줍니다.