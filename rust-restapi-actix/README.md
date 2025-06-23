# Actix-web JWT REST API 예제

이 프로젝트는 Rust의 `actix-web` 프레임워크를 사용하여 기본적인 JWT(JSON Web Token) 인증 REST API를 구현한 예제입니다.

## 주요 기능

-   **JWT 발급**: 사용자 ID를 받아 Access Token과 Refresh Token을 생성합니다.
-   **JWT 갱신**: 유효한 Refresh Token으로 새로운 Access Token을 발급합니다.
-   **JWT 검증**: Access Token의 유효성을 검증합니다.

## 기술 스택

-   [Rust](https://www.rust-lang.org/)
-   [Actix-web](https://actix.rs/): 웹 프레임워크
-   [jsonwebtoken](https://crates.io/crates/jsonwebtoken): JWT 생성 및 검증
-   [Serde](https://serde.rs/): JSON 직렬화/역직렬화
-   [Chrono](https://crates.io/crates/chrono): 시간 및 날짜 처리

## 시작하기

### 1. 전제 조건

-   [Rust 프로그래밍 언어](https://www.rust-lang.org/tools/install)가 설치되어 있어야 합니다.

### 2. 프로젝트 실행

1.  이 저장소를 클론하거나 코드를 다운로드합니다.
2.  프로젝트 루트 디렉토리에서 다음 명령어를 실행하여 의존성을 다운로드하고 서버를 시작합니다.

    ```bash
    cargo run
    ```

3.  서버가 성공적으로 시작되면 터미널에 다음 메시지가 표시되고, `http://127.0.0.1:8080`에서 요청을 기다립니다.

    ```
    Starting server at http://127.0.0.1:8080
    ```

## API 엔드포인트 테스트

`curl`과 같은 도구를 사용하여 API를 테스트할 수 있습니다.

### 1. JWT 발급 (`/issue-jwt`)

사용자 ID (`user123`)로 1분 유효 기간의 Access Token과 1시간 유효 기간의 Refresh Token을 요청합니다.

```bash
curl -X POST http://127.0.0.1:8080/issue-jwt \
-H "Content-Type: application/json" \
-d '{"user_id": "user123"}'
```

**응답 예시:**

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiJ9..."
}
```

> **참고**: 다음 단계에서 사용할 수 있도록 `access_token`과 `refresh_token` 값을 복사해 두세요.

### 2. JWT 검증 (`/verify-jwt`)

발급받은 `access_token`의 유효성을 검증합니다.

`YOUR_ACCESS_TOKEN`을 위에서 받은 토큰 값으로 교체하세요.

```bash
curl -X POST http://127.0.0.1:8080/verify-jwt \
-H "Content-Type: application/json" \
-d '{"token": "YOUR_ACCESS_TOKEN"}'
```

**응답 예시 (유효할 경우):**

```json
{
  "valid": true,
  "claims": {
    "sub": "user123",
    "exp": 1678886460,
    "iat": 1678886400,
    "token_type": "access"
  }
}
```

> 이 API는 `token_type`이 "access"인 토큰만 유효한 것으로 간주합니다. Refresh Token으로 시도하면 `valid: false`가 반환됩니다.

### 3. JWT 갱신 (`/refresh-jwt`)

만료된 Access Token을 갱신하기 위해 `refresh_token`을 사용합니다.

`YOUR_REFRESH_TOKEN`을 1단계에서 받은 토큰 값으로 교체하세요.

```bash
curl -X POST http://127.0.0.1:8080/refresh-jwt \
-H "Content-Type: application/json" \
-d '{"refresh_token": "YOUR_REFRESH_TOKEN"}'
```

**응답 예시:**

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiJ9..."
}
```

> 이 API는 `token_type`이 "refresh"인 토큰만 허용합니다.

## 보안 고려 사항

-   **비밀 키 관리**: 현재 코드의 `JWT_SECRET`은 예제용으로 하드코딩되어 있습니다. 실제 애플리케이션에서는 환경 변수, Vault, 또는 다른 보안 저장소를 통해 안전하게 관리해야 합니다.
-   **Token Type**: Access Token과 Refresh Token의 오용을 방지하기 위해 각 토큰의 `Claims`에 `token_type` 필드를 두어 용도를 명확히 구분하고, 각 API 핸들러에서 이를 검증합니다. 