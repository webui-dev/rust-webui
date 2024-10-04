use webui_rs::{webui::wait, window::Window};

fn main() {
    let win = Window::new();

    win.show("examples/html/index.html");

    wait();
}
