# Rust-Tutorial

## What is Rust?
- Rust is a statically compiled language in a similar role as C++
  - `rustc` uses **LLVM** as its backend.
- 
- Rust supports many platforms and architectures:
  - x86, ARM, WebAssembly, …
  - Linux, Mac, Windows, …
- 
- Rust is used for a wide range of devices:
  - firmware and boot loaders,
  - smart displays,
  - mobile phones,
  - desktops,
  - servers.

### Rust 구성 요소
- `rustc` : the **Rust compiler** which turns .rs files into binaries and other intermediate formats.
- `Cargo` : the Rust dependency manager and build tool
  - https://crates.io/ 에서 dependency 다운로드
  - 프로젝트 빌드시 `rustc` 에 전달
- `Rustfmt` : Rust formatter
- `Rust Language Server` : 코드 자동완성Code completion , inline-error
  메시지를 위한 IDE 결합

### Install
- rustup : the Rust toolchain installer and updater
  - rust 버전관리 가능
  - rust 새로운 release 시, rustc, cargo 업데이트에 사용

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


### Cargo Run
```
$ cargo new <folder name>

# Build binary
$ cargo run 

# check for error
$ cargo check

# build without runnung
$ cargo build
```
