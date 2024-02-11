use webui_rs::webui::{wait, Window};

pub fn main() {
  let win = Window::new();

  win.show("examples/html/index.html");

  wait();
}