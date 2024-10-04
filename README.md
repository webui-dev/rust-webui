<div align="center">

![Logo](https://raw.githubusercontent.com/webui-dev/webui-logo/main/webui_rust.png)

# Rust-WebUI

[last-commit]: https://img.shields.io/github/last-commit/webui-dev/rust-webui?style=for-the-badge&logo=github&logoColor=C0CAF5&labelColor=414868
[license]: https://img.shields.io/github/license/webui-dev/rust-webui?style=for-the-badge&logo=opensourcehardware&label=License&logoColor=C0CAF5&labelColor=414868&color=8c73cc
[![][last-commit]](https://github.com/webui-dev/rust-webui/pulse)
[![][license]](https://github.com/webui-dev/rust-webui/blob/main/LICENSE)

> Use any web browser or WebView as GUI, with Rust in the backend and modern web technologies in the frontend, all in a lightweight portable library.

![Screenshot](https://raw.githubusercontent.com/webui-dev/webui-logo/main/screenshot.png)

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
  webui-rs = { git = "https://github.com/webui-dev/rust-webui/", branch = "main" }

  # Or by git tag
  webui-rs = { git = "https://github.com/webui-dev/rust-webui/", tag = "v2.4.2" }

  # Or by git commit
  webui-rs = { git = "https://github.com/webui-dev/rust-webui/", rev = "a1b2c3d4" }
  ```

2. Then bring in the static [WebUI static release](https://github.com/webui-dev/webui/releases) or [build action](https://github.com/webui-dev/webui/actions?query=branch%3Amain) file for your platform and place it in your project's root directory.

3. That's it!

## Usage

```rust
use webui_rs::webui;

pub fn main() {
  let win = webui::Window::new();
  win.show("<html><body><h1>Hello, World!</h1></body></html>");
  webui::wait();
}
```

## UI & The Web Technologies

[Borislav Stanimirov](https://ibob.bg/) discusses using HTML5 in the web browser as GUI at the [C++ Conference 2019 (_YouTube_)](https://www.youtube.com/watch?v=bbbcZd4cuxg).

<!-- <div align="center">
  <a href="https://www.youtube.com/watch?v=bbbcZd4cuxg"><img src="https://img.youtube.com/vi/bbbcZd4cuxg/0.jpg" alt="Embrace Modern Technology: Using HTML 5 for GUI in C++ - Borislav Stanimirov - CppCon 2019"></a>
</div> -->

<div align="center">

![CPPCon](https://github.com/webui-dev/webui/assets/34311583/4e830caa-4ca0-44ff-825f-7cd6d94083c8)

</div>

Web application UI design is not just about how a product looks but how it works. Using web technologies in your UI makes your product modern and professional, And a well-designed web application will help you make a solid first impression on potential customers. Great web application design also assists you in nurturing leads and increasing conversions. In addition, it makes navigating and using your web app easier for your users.

### Why Use Web Browsers?

Today's web browsers have everything a modern UI needs. Web browsers are very sophisticated and optimized. Therefore, using it as a GUI will be an excellent choice. While old legacy GUI lib is complex and outdated, a WebView-based app is still an option. However, a WebView needs a huge SDK to build and many dependencies to run, and it can only provide some features like a real web browser. That is why WebUI uses real web browsers to give you full features of comprehensive web technologies while keeping your software lightweight and portable.

### How Does it Work?

<div align="center">

![Diagram](https://github.com/ttytm/webui/assets/34311583/dbde3573-3161-421e-925c-392a39f45ab3)

</div>

Think of WebUI like a WebView controller, but instead of embedding the WebView controller in your program, which makes the final program big in size, and non-portable as it needs the WebView runtimes. Instead, by using WebUI, you use a tiny static/dynamic library to run any installed web browser and use it as GUI, which makes your program small, fast, and portable. **All it needs is a web browser**.

### Runtime Dependencies Comparison

|                                 | Tauri / WebView   | Qt                         | WebUI               |
| ------------------------------- | ----------------- | -------------------------- | ------------------- |
| Runtime Dependencies on Windows | _WebView2_        | _QtCore, QtGui, QtWidgets_ | **_A Web Browser_** |
| Runtime Dependencies on Linux   | _GTK3, WebKitGTK_ | _QtCore, QtGui, QtWidgets_ | **_A Web Browser_** |
| Runtime Dependencies on macOS   | _Cocoa, WebKit_   | _QtCore, QtGui, QtWidgets_ | **_A Web Browser_** |

## Supported Web Browsers

| Browser         | Windows         | macOS         | Linux           |
| --------------- | --------------- | ------------- | --------------- |
| Mozilla Firefox | ✔️              | ✔️            | ✔️              |
| Google Chrome   | ✔️              | ✔️            | ✔️              |
| Microsoft Edge  | ✔️              | ✔️            | ✔️              |
| Chromium        | ✔️              | ✔️            | ✔️              |
| Yandex          | ✔️              | ✔️            | ✔️              |
| Brave           | ✔️              | ✔️            | ✔️              |
| Vivaldi         | ✔️              | ✔️            | ✔️              |
| Epic            | ✔️              | ✔️            | _not available_ |
| Apple Safari    | _not available_ | _coming soon_ | _not available_ |
| Opera           | _coming soon_   | _coming soon_ | _coming soon_   |

### Tests

```
cargo test --package webui-rs --lib -- tests --show-output
```

### License

> Licensed under the MIT License.
