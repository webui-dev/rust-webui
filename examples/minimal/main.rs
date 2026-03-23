// WebUI Rust - Minimal Example

use webui_rs::webui;

fn main() {
    let window = webui::Window::new();
    window.show(
        "<html><head><script src=\"webui.js\"></script></head><body>Hello World!</body></html>",
    );
    webui::wait();
}
