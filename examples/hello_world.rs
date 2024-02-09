use webui_rs::webui::{Window, wait};

fn main() {
    let win = Window::new();
    win.show("<html>Hello World!</html>");

    wait();
}