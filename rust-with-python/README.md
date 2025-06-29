# Python with Rust: 소수 합계 계산 예제

이 프로젝트는 Python과 Rust를 함께 사용하여 성능이 중요한 부분을 Rust로 구현하고, 메인 로직은 Python으로 제어하는 방법을 보여주는 예제입니다. "주어진 숫자까지의 모든 소수의 합을 계산하는" 작업을 통해 두 언어의 성능 차이를 명확하게 확인할 수 있습니다.

## ✨ 주요 특징

- **성능 병목 현상 해결**: 계산 집약적인 작업을 Rust로 작성하여 Python의 단점을 보완합니다.
- **간편한 인터페이스**: Python의 편리한 문법과 생태계는 그대로 유지합니다.
- **PyO3 & Maturin**: `PyO3`를 통해 Rust와 Python 간의 바인딩을 생성하고, `maturin`으로 빌드 및 패키징 과정을 자동화합니다.

## 📂 프로젝트 구조

```
.
├── rust_lib/                # Rust 라이브러리 소스 코드
│   ├── src/
│   │   └── lib.rs           # 소수 합계 계산 Rust 로직
│   └── pyproject.toml       # Rust 라이브러리 빌드 설정
├── .venv/                   # Python 가상 환경
├── main.py                  # 메인 Python 스크립트
└── README.md                # 프로젝트 설명 파일
```

## ⚙️ 동작 원리

Python이 어떻게 Rust로 작성된 함수를 호출할 수 있는지에 대한 과정은 다음과 같습니다.

### 1. Rust 코드 작성 및 Python 노출 (`PyO3`)

Rust 코드(`rust_lib/src/lib.rs`)에서 `PyO3` 라이브러리를 사용하여 Python에 노출할 함수를 정의합니다.

- `#[pyfunction]`: Rust 함수를 Python에서 호출 가능한 형태로 변환합니다. 데이터 타입(예: Python `int` ↔ Rust `u64`) 변환도 자동으로 처리됩니다.
- `#[pymodule]`: Python에서 `import`할 모듈을 정의합니다. 이 모듈 안에 위에서 정의한 함수들을 추가합니다.

```rust
// rust_lib/src/lib.rs
use pyo3::prelude::*;

// 이 함수는 Python에 노출됩니다.
#[pyfunction]
fn sum_primes(limit: u64) -> PyResult<u64> {
    // ... 소수 합계 계산 로직 ...
}

// "rust_lib" 라는 Python 모듈을 생성합니다.
#[pymodule]
fn rust_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_primes, m)?)?;
    Ok(())
}
```

### 2. Rust 코드를 공유 라이브러리로 컴파일 (`maturin`)

`maturin`은 Rust 코드를 Python이 이해할 수 있는 **공유 라이브러리**(`.so` 또는 `.pyd` 파일)로 컴파일하고, 이를 현재 Python 가상 환경에 설치하는 역할을 합니다.

이 과정은 터미널에서 아래 명령어를 실행하는 것으로 간단하게 완료됩니다.

```bash
# rust_lib 디렉토리 안에서 실행
maturin develop
```

`maturin`은 `pyproject.toml` 설정을 읽어 Rust 프로젝트를 빌드하고, 그 결과물을 `pip`으로 패키지를 설치한 것과 같은 위치(`.venv/lib/site-packages`)에 놓아줍니다.

### 3. Python에서 모듈 사용

컴파일 및 설치가 완료되면, Python 코드는 `rust_lib`를 일반적인 Python 패키지처럼 `import`하여 사용할 수 있습니다.

```python
# main.py

# Rust로 빌드된 모듈을 가져옵니다.
import rust_lib
import time

# 사용자로부터 숫자를 입력받습니다.
limit = 1_000_000

# Rust 함수를 호출하고 시간을 측정합니다.
start_time = time.time()
result = rust_lib.sum_primes(limit) # 네이티브 속도로 실행!
duration = time.time() - start_time

print(f"[Rust] 결과: {result}, 시간: {duration:.4f}초")
```

Python 인터프리터는 `import rust_lib` 구문을 만나면 `site-packages`에 설치된 공유 라이브러리를 찾아 로드합니다. 이제 우리는 Rust의 성능을 Python의 편리함과 함께 활용할 수 있습니다.

## 🚀 실행 방법

1.  **사전 준비**:
    - [Python](https://www.python.org/) 3.x 버전 설치
    - [Rust](https://www.rust-lang.org/tools/install) 설치

2.  **프로젝트 설정**:
    ```bash
    # Python 가상 환경 생성 및 활성화
    uv init
    ```

3.  **의존성 설치**:
    ```bash
    # maturin 설치
    uv add maturin
    ```

4.  **Rust 모듈 빌드**:
    ```bash
    # rust_lib 디렉토리로 이동하여 빌드
    cd rust_lib
    maturin develop
    cd ..
    ```

5.  **Python 스크립트 실행**:
    ```bash
    python main.py
    ```
    실행 후 숫자를 입력하면 Rust와 Python의 계산 속도를 비교하는 결과가 출력됩니다.

## 📚 다양한 데이터 타입 다루기

`PyO3`는 `u64`와 같은 단순한 숫자 타입 외에도 문자열, 리스트, 딕셔너리, 구조체 등 복잡한 데이터 타입을 Python과 Rust 간에 주고받을 수 있는 강력한 기능을 제공합니다.

### 1. 문자열 (String)

Python의 `str`은 Rust의 `String` 또는 `&str`과 자동으로 변환됩니다.

**Rust 예시 (`lib.rs`):**
```rust
#[pyfunction]
fn process_text(text: String) -> PyResult<String> {
    let processed_text = format!("Rust가 처리함: {}", text.to_uppercase());
    Ok(processed_text)
}
```

### 2. 리스트 (List)

Python의 `list`는 Rust의 `Vec<T>` 타입으로 변환할 수 있습니다. `T`에는 `PyO3`가 변환할 수 있는 모든 타입(예: `i64`, `String`, `bool`)이 올 수 있습니다.

**Rust 예시 (`lib.rs`):**
```rust
#[pyfunction]
fn sum_list_of_numbers(numbers: Vec<i64>) -> PyResult<i64> {
    let sum: i64 = numbers.iter().sum();
    Ok(sum)
}
```

### 3. 딕셔너리 (Dictionary) 및 구조체 (Struct)

Python의 `dict`를 다루는 방법은 크게 두 가지가 있습니다.

#### 방법 A: `PyDict` 타입으로 직접 접근

`pyo3::types::PyDict` 타입을 사용하여 Python 딕셔너리를 직접 다룰 수 있습니다. 유연하지만 타입과 키 존재 여부를 직접 확인해야 하는 번거로움이 있습니다.

**Rust 예시 (`lib.rs`):**
```rust
use pyo3::types::PyDict;

#[pyfunction]
fn greet_from_dict(user_info: &Bound<'_, PyDict>) -> PyResult<String> {
    let name: String = user_info.get_item("name")?.expect("'name' 키가 필요합니다.").extract()?;
    let age: i32 = user_info.get_item("age")?.expect("'age' 키가 필요합니다.").extract()?;

    Ok(format!("안녕하세요, {}님! {}살이시군요.", name, age))
}
```

#### 방법 B: Rust 구조체를 Python 클래스로 변환 (권장)

Rust의 `struct`에 `#[pyclass]` 어트리뷰트를 붙여 Python 클래스로 직접 변환할 수 있습니다. 이 방법은 코드를 더 구조적이고 안정적으로 만들어주며, Python에서 객체 지향적으로 다룰 수 있게 해줍니다.

**Rust 예시 (`lib.rs`):**
```rust
#[pyclass]
#[derive(Clone)] // Python에서 객체를 복사할 수 있게 하려면 필요
struct User {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    age: u32,
}

#[pymethods]
impl User {
    #[new] // Python의 __init__ 생성자
    fn new(name: String, age: u32) -> Self {
        User { name, age }
    }

    fn greet(&self) -> String {
        format!("제 이름은 {}이고, 나이는 {}세입니다.", self.name, self.age)
    }
}
```
위와 같이 정의된 `User` 클래스는 Python에서 `user = rust_lib.User("Alice", 30)` 처럼 생성하고, `user.name`으로 속성에 접근하거나 `user.greet()`와 같은 메서드를 호출할 수 있습니다. 모듈에 클래스를 추가하려면 `#[pymodule]` 함수 내에 `m.add_class::<User>()?` 코드를 추가해야 합니다.

## 🤖 CI/CD 자동화: `CI.yml` 파일의 역할

`maturin new` 명령어를 실행하면 `rust_lib/.github/workflows/CI.yml` 경로에 파일이 자동으로 생성되는 것을 볼 수 있습니다. 이 파일은 **프로젝트의 빌드, 테스트, 배포 과정을 자동화**하기 위한 [GitHub Actions](https://github.com/features/actions) 워크플로우 설정입니다.

### 주요 기능

1.  **지속적 통합 (Continuous Integration)**:
    - GitHub 저장소에 코드가 푸시(push)될 때마다, `Linux`, `Windows`, `macOS` 등 여러 운영체제와 CPU 아키텍처 환경에서 Rust 코드가 정상적으로 빌드되는지 자동으로 검사합니다.
    - 이를 통해 다양한 환경에서의 호환성을 보장하고 코드 변경으로 인한 실수를 조기에 발견할 수 있습니다.

2.  **지속적 배포 (Continuous Deployment)**:
    - 프로젝트에 새로운 버전 태그(예: `v1.0.1`)를 생성하여 푸시하면, 워크플로우가 자동으로 각 환경에 맞는 패키지를 빌드합니다.
    - 빌드된 결과물(컴파일된 **Wheel** 파일과 소스 배포본)을 **PyPI(Python Package Index)**에 업로드하여 다른 사람들이 `pip install` 명령어로 쉽게 설치할 수 있도록 합니다.

### 배포의 범위

이 워크플로우가 배포하는 것은 **`rust_lib` 라이브러리**입니다. 즉, 재사용 가능한 "부품"으로서의 Rust 모듈만 패키징하여 배포합니다.

`main.py`와 같은 애플리케이션 코드는 `rust_lib` 라이브러리를 사용하는 "예시"이며, 배포 패키지에는 포함되지 않습니다.

> **참고**: `maturin`은 이 워크플로우 파일을 `rust_lib` 디렉토리 내에 생성합니다. 만약 이 프로젝트를 독립적인 Git 저장소로 관리하고 실제 CI/CD를 실행하려면, `.github` 디렉토리를 프로젝트의 최상위 루트로 이동해야 합니다. (이 예제에서는 불필요합니다.)
