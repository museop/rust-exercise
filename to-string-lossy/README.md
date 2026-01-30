# `to_string_lossy().into_owned()` 패턴

이 패턴은 Rust에서 **파일 시스템 경로(Path/PathBuf)** 나 FFI(외부 함수 인터페이스), 혹은 운영체제와 상호작용할 때 매우 자주 등장합니다.

이 코드가 필요한 핵심 이유는 **"모든 바이트열이 유효한 UTF-8 문자열은 아니기 때문"** 입니다.

`OsStr` / `Path`: 운영체제마다 문자열을 다루는 방식이 다릅니다(Linux는 바이트 배열, Windows는 UTF-16 등). Rust의 String은 무조건 UTF-8이어야 하지만, 운영체제에서 온 데이터는 UTF-8이 아닐 수도 있습니다.


`to_string_lossy()`:

- UTF-8로 변환을 시도합니다.
- 성공 시: 원본 데이터를 빌려옵니다 (비용 저렴).
- 실패 시 (깨진 문자): 깨진 부분을 `U+FFFD Replacement Character`로 바꾸고 새 문자열을 만듭니다.
  - 반환 타입은 `Cow<str>` (Clone on Write)입니다.

`into_owned()`:
- Cow가 빌리고 있던, 새로 만들었던 상관없이 **"나만의 소유권이 있는 String"** 으로 확정 짓습니다.