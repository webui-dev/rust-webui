<div align="center">

# WebUI Rust

[last-commit]: https://img.shields.io/github/last-commit/webui-dev/rust-webui?style=for-the-badge&logo=github&logoColor=C0CAF5&labelColor=414868
[license]: https://img.shields.io/github/license/webui-dev/rust-webui?style=for-the-badge&logo=opensourcehardware&label=License&logoColor=C0CAF5&labelColor=414868&color=8c73cc
[![][last-commit]](https://github.com/webui-dev/rust-webui/pulse)
[![][license]](https://github.com/webui-dev/rust-webui/blob/main/LICENSE)

> WebUI is not a web-server solution or a framework, but it allows you to use any web browser as a GUI, with your preferred language in the backend and HTML5 in the frontend. All in a lightweight portable lib.

![Screenshot](https://github.com/webui-dev/webui/assets/34311583/57992ef1-4f7f-4d60-8045-7b07df4088c6)

</div>

## Features

* Parent library written in pure C
* Lightweight ~200 Kb & Small memory footprint
* Fast binary communication protocol between WebUI and the browser (Instead of JSON)
* Multi-platform & Multi-Browser
* Using private profile for safety

## Examples

Examples are found in `examples/` and can be run with `cargo run --example <example_name>`.

## Installation

1. Add `webui` to your dependencies in `Cargo.toml`:
  ```toml
  webui = { git = "https://github.com/webui-dev/rust-webui/", branch = "main" }
  ```

2. Then bring in a static [WebUI release .lib](https://github.com/webui-dev/webui/releases) file to your project and link it with your Rust code using [`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html):

```rust
fn main() {
    println!("cargo:rustc-link-lib=webui-2-static");
}
```