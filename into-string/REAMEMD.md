# Rust의 `impl Into<String>`와 소유권(Ownership) 정리

`impl Into<String>`는 **다양한 문자열 타입(`String`, `&str`, 등)** 을 하나의 API에서 받을 수 있게 해주는 편리한 제네릭 인터페이스입니다.  
하지만 `Into::into(self)`는 **`self`의 소유권을 가져가서** 변환을 수행하므로, 사용 시 소유권 및 성능 특성을 이해하는 것이 중요합니다.

---

## 1. `Into` / `From` 관계

```rust
pub trait Into<T> {
    fn into(self) -> T;
}
```

- `From<T> for U`를 구현하면 자동으로 `Into<U> for T`가 구현됩니다.
- String 관련 주요 구현:
    - `From<String> for String` → 단순 이동(move)
    - `From<&str> for String` → 새로 String 생성(할당 발생)

## 2. 핵심: `into(self)`는 소유권을 가져감
- `self`가 `String` → 이동(move) 발생, 재할당 없음
- `self`가 `&str` → 참조를 복사하여 새로운 `String`을 생성 (할당 & 복사 발생)

## 3. 설계 시 권장 사항
- 함수가 문자열을 소유해야 한다면 `impl Into<String>` 또는 `S: Into<String>` 사용 → 호출자는 `String`과 `&str` 모두 전달 가능
- 읽기만 한다면 `&str` 또는 `impl AsRef<str>` 고려
- 불필요한 복사 피하기
    - `String` → `move`만 발생 (재할당 없음)
    - `&str` → 새로 할당 & 복사 발생
    - `&String` → 보통 &str로 변환 후 처리


## 4. 성능 요약

| 변환	| 소유권	| 할당 발생 여부 |
|------|---------|-------------|
| `String` → `String` | 이동	|❌ 없음|
| `&str` → `String` |	복사	|✅ 있음|
| `&String` → `String`	| 복사	| ✅ 있음|
------------------------------


## 참고
- `Into`는 값(`self`)의 소유권을 가져갑니다.
- 불필요한 복사와 할당을 피하려면 API의 소유권 요구 여부를 먼저 결정하세요.