# Rust-LS (Simple LS Implementation)

Rust로 작성된 간단한 `ls -al` 구현체입니다. 유닉스 시스템(Linux, macOS)의 파일 메타데이터를 읽어들여, 사람이 읽기 쉬운 형식으로 디렉토리 목록을 출력합니다.

## 주요 기능 (Features)
- 디렉토리 조회: 현재 디렉토리 또는 인자로 전달된 경로의 파일 목록을 보여줍니다.
- 상세 정보 표시 (`-al` 스타일):
- 파일 유형 (디렉토리 `d`, 심볼릭 링크 `l`, 파일 `-`)
- 파일 권한 (예: `rwxr-xr-x`)
- 하드 링크 수
- 소유자(User) 및 그룹(Group) 이름 (UID/GID 변환)
- 파일 크기 (Human Readable 포맷 지원: `B`, `K`, `M`, `G`...)
- 수정 시간 (월 일 시:분)
- 심볼릭 링크 추적: 심볼릭 링크일 경우 원본 경로를 화살표(`->`)로 표시합니다.
- 이름순 정렬: 파일 이름을 기준으로 오름차순 정렬하여 출력합니다.

## 설치 및 실행 (Installation & Usage)

1. 저장소 클론 및 이동: 프로젝트 폴더로 이동합니다.
2. 빌드 및 실행: cargo run 명령어로 바로 실행할 수 있습니다.

현재 디렉토리 조회:
```sh
cargo run
```

특정 경로 조회 (예: `/etc`):
```sh
cargo run -- /etc
```

3. 시스템에 설치 (선택 사항): 어디서든 rust-ls 명령어로 사용하고 싶다면 설치할 수 있습니다.
```
cargo install --path .
```

설치 후 사용법:
```
rust-ls
rust-ls /usr/bin
```

출력 예시 (Output Example):
터미널에서 다음과 같은 형식으로 출력됩니다.

```
drwxr-xr-x  19 museop   staff      608B  1월 30 11:45 .
drwxr-xr-x   7 museop   staff      224B  1월 29 23:35 ..
-rw-r--r--   1 museop   staff      1.2K  1월 30 10:00 Cargo.toml
drwxr-xr-x   4 museop   staff      128B  1월 30 10:00 src
lrwxr-xr-x   1 museop   staff       12B  1월 30 11:50 mylink -> target/debug
```