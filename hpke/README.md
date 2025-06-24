# HPKE (Hybrid Public Key Encryption) 예제

이 프로젝트는 Rust로 구현된 HPKE(Hybrid Public Key Encryption)의 간단한 사용법을 보여주는 예제입니다. `sender`와 `receiver`라는 두 개의 바이너리를 통해 메시지를 암호화하고 복호화하는 과정을 시뮬레이션합니다.

## HPKE의 구성 요소

HPKE(Hybrid Public Key Encryption, 하이브리드 공개키 암호화)는 [RFC 9180](https://www.rfc-editor.org/rfc/rfc9180.html)으로 표준화된 최신 암호화 체계입니다. 이름처럼 비대칭 암호화와 대칭 암호화를 결합한 방식으로, 다음과 같은 세 가지 주요 암호화 기본 요소(primitive)의 조합으로 정의됩니다.

1.  **KEM (Key Encapsulation Mechanism, 키 캡슐화 메커니즘)**
    공개키 암호를 사용하여 '공유 비밀(shared secret)'을 설정하는 절차입니다. 이 공유 비밀은 실제 메시지를 암호화하는 대칭키를 생성하는 재료가 됩니다. HPKE에서는 주로 디피-헬만(Diffie-Hellman) 키 교환 방식을 사용합니다. (자세한 과정은 아래 "핵심 동작 원리: 키 유도 과정"에서 설명합니다.)

2.  **KDF (Key Derivation Function, 키 유도 함수)**
    하나의 비밀(KEM을 통해 얻은 공유 비밀)로부터 암호학적으로 안전한 여러 개의 키를 생성하는 함수입니다. HPKE는 KEM이 만든 공유 비밀을 KDF에 입력하여, 실제 데이터를 암호화할 대칭키, 추가 데이터(AAD)를 인증할 키 등 세션에 필요한 다양한 키들을 '유도'해냅니다.

3.  **AEAD (Authenticated Encryption with Associated Data, 연관 데이터를 사용한 인증 암호화)**
    실제 메시지를 암호화하는 대칭키 암호화 방식입니다. AEAD는 다음과 같은 세 가지 중요한 보안 속성을 동시에 제공합니다.
    - **기밀성(Confidentiality)**: 허가된 사용자 외에는 메시지 내용을 볼 수 없습니다.
    - **무결성(Integrity)**: 메시지가 전송 중에 변경되지 않았음을 보장합니다.
    - **인증(Authenticity)**: 메시지가 올바른 송신자로부터 왔음을 확인합니다.
    
    또한, 암호화되지 않지만 무결성은 검증되어야 하는 '연관 데이터(Associated Data, AD)'를 함께 처리할 수 있는 특징이 있습니다.

이 세 요소를 결합하여, HPKE는 공개키 방식의 안전성 위에서 대칭키 방식의 효율성으로 데이터를 암호화하는 강력한 보안을 제공합니다.

## 핵심 동작 원리: 키 '유도' 과정

`sender`가 `Encapsulated Key`를 "만든다"기보다, `sender`와 `receiver`가 이 키를 포함한 정보를 바탕으로 동일한 세션 키를 각자 '유도'한다는 표현이 더 정확합니다. 이 과정은 디피-헬만 키 교환의 원리를 따르며, 네트워크상에 세션 키 자체가 오고 가지 않아 매우 안전합니다.

1.  **준비 (Receiver)**: `receiver`는 자신의 장기적인 개인키(`sk_r`)와 공개키(`pk_r`) 쌍을 가지고 있습니다.
2.  **캡슐화 (Sender)**
    - `sender`는 메시지를 보낼 때마다 **임시(ephemeral) 개인키(`sk_e`)와 공개키(`pk_e`) 쌍**을 새로 생성합니다.
    - `sender`는 자신의 임시 개인키(`sk_e`)와 `receiver`의 공개키(`pk_r`)를 사용하여 **공유 비밀(shared secret)**을 계산합니다.
    - `sender`는 이 임시 공개키(`pk_e`)를 `receiver`에게 전송합니다. 이것이 바로 **`Encapsulated Key`**입니다.
3.  **복호화 (Receiver)**
    - `receiver`는 `sender`로부터 `Encapsulated Key`(`pk_e`)를 받습니다.
    - `receiver`는 자신의 개인키(`sk_r`)와 `sender`가 보낸 임시 공개키(`pk_e`)를 사용하여 **동일한 공유 비밀**을 계산합니다.
4.  **키 유도 (양측 동일)**
    - `sender`와 `receiver`는 이제 누구에게도 전송된 적 없는 동일한 '공유 비밀'을 갖게 되었습니다.
    - 양측은 이 공유 비밀을 KDF에 넣어 실제 메시지를 암호화하고 복호화하는 데 사용할 세션 키(들)를 동일하게 유도해냅니다.

결론적으로, `Encapsulated Key`는 세션 키 자체가 아니라, `receiver`가 `sender`와 동일한 공유 비밀을 유도하는 데 필요한 공개 정보입니다. 이 과정을 통해 실제 암호화 키는 외부에 노출되지 않으므로 매우 높은 수준의 보안이 달성됩니다.

## 프로젝트 구조 및 동작 원리

이 프로젝트는 두 개의 독립적인 실행 파일, `sender`와 `receiver`로 구성됩니다.

### `receiver`

1.  **키 생성**: 프로그램을 시작하면 먼저 `X25519HkdfSha256` KEM을 사용하여 자신의 개인키(`sk_r`)와 공개키(`pk_r`) 쌍을 생성합니다.
2.  **공개키 공유**: 생성된 공개키(`pk_r`)를 16진수(hex) 형태로 콘솔에 출력합니다. 이 공개키는 `sender`가 메시지를 암호화하는 데 사용됩니다.
3.  **대기**: `sender`로부터 전달될 `Encapsulated Key`와 `Ciphertext` 입력을 기다립니다.
4.  **복호화 컨텍스트 설정**: 자신의 개인키(`sk_r`)와 `sender`가 보낸 `Encapsulated Key`를 사용하여 HPKE 수신 컨텍스트를 설정합니다. 이 과정을 통해 `sender`와 공유하는 대칭 세션 키를 안전하게 유도합니다.
5.  **메시지 복호화**: 설정된 컨텍스트를 사용하여 `Ciphertext`를 복호화하고 원본 메시지를 복원합니다.
6.  **결과 출력**: 성공적으로 복호화된 메시지를 콘솔에 출력합니다.

### `sender`

1.  **입력 받기**: 커맨드 라인 인자(CLI)로 `receiver`의 공개키(`pk_r`)와 암호화할 메시지를 입력받습니다.
2.  **암호화 컨텍스트 설정**: 입력받은 `receiver`의 공개키를 사용하여 HPKE 송신 컨텍스트를 설정합니다. 이 과정에서 임시 키 쌍이 생성되고, 이를 `receiver`의 공개키와 결합하여 대칭 세션 키를 유도합니다. 이 임시 공개키가 바로 `Encapsulated Key`가 되어 `receiver`에게 전달됩니다.
3.  **메시지 암호화**: 설정된 컨텍스트를 사용하여 메시지를 암호화(`seal`)하여 `Ciphertext`를 생성합니다.
4.  **결과 출력**: `Encapsulated Key`와 `Ciphertext`를 16진수 형태로 콘솔에 출력합니다. 이 값들은 `receiver`에게 전달되어야 합니다.

## 실행 방법

두 개의 터미널 창을 열어 진행합니다.

#### 1. 프로젝트 빌드

먼저 `cargo`를 사용하여 프로젝트를 빌드합니다.

```bash
cargo build
```

#### 2. `receiver` 실행 (첫 번째 터미널)

첫 번째 터미널에서 `receiver`를 실행합니다. `receiver`는 자신의 공개키를 출력하고 입력을 기다립니다.

```bash
cargo run --bin receiver
```

실행 결과:
```
Receiver's Public Key (hex): [수신자의-공개키-문자열]
--------------------------------------------------
Waiting for sender's output...
Enter Encapsulated Key (hex):
```
`Receiver's Public Key (hex):` 옆의 키 문자열을 복사합니다.

#### 3. `sender` 실행 (두 번째 터미널)

두 번째 터미널에서 `sender`를 실행합니다.
- `<receiver의-공개키>` 부분에 위 단계에서 복사한 키를 붙여넣습니다.
- `<"보낼 메시지">` 부분에 암호화할 메시지를 입력합니다.

```bash
cargo run --bin sender -- --public-key-hex <receiver의-공개키> --message "Hello, HPKE!"
```

실행 결과:
```
Encapsulated Key (hex): [캡슐화된-키-문자열]
Ciphertext (hex): [암호문-문자열]
```
위 두 줄의 16진수 문자열을 복사합니다.

#### 4. 메시지 복호화 (다시 첫 번째 터미널)

`receiver`가 실행 중인 첫 번째 터미널로 돌아와 `sender`가 출력한 값들을 순서대로 입력합니다.

1. `Encapsulated Key` 값을 붙여넣고 `Enter` 키를 누릅니다.
2. `Ciphertext` 값을 붙여넣고 `Enter` 키를 누릅니다.

`receiver`는 즉시 암호문을 복호화하여 원본 메시지를 출력합니다.

```
Successfully decrypted: Hello, HPKE! 