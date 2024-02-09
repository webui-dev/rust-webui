use webui_rs::webui::Window;

fn main() {
    let win = Window::new();
    win.show("<html>Hello World!</html>");

    win.wait();
}