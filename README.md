# Rust-Tutorial

### Rust 구성 요소
- Cargo : dependency 관리자 및 빌드 도구
- Rustfmt : Rust formatter
- Rust Language Server : 코드 자동완성Code completion , inline-error
  메시지를 위한 IDE 결합

### Install
- rustup : Rust 버전 관리 도구

**MacOS / Linux**
- 스크립트 다운로드
```bash
$ curl https://sh.rustup.rs -sSf | sh
```

환경 변수 추가
```bash
$ export PATH="$HOME/.cargo/bin:$PATH"
```

설치 확인
```bash
$ rustc --version
```