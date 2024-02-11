use webui_rs::webui::{wait, Window};

fn main() {
    let win = Window::new();

    win.show("examples/html/index.html");

    wait();
}
