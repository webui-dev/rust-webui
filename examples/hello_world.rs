use webui_rs::webui::{Window, wait as webui_wait};

fn main() {
    let win = Window::new();
    win.show("<html>Hello World!</html>");

    webui_wait();
}