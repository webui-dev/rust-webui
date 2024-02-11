use webui_rs::webui::{wait, Event, Window};

fn main() {
  let win = Window::new();

  // Inline function
  win.bind("my_button", |_: Event| {
    println!("Button clicked!");
  });

  win.show(r#"
  <html>
    <script src="/webui.js"></script>
    <button id="my_button">Click me for some backend logs!</button>
  </html>
  "#);

  wait();
}