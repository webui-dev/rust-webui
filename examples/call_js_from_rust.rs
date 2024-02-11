use webui_rs::webui::{wait, Event, JavaScript, Window};

fn main() {
    let win = Window::new();

    // Declared function
    win.bind("my_button", log_to_js);

    win.show(
        r#"
  <html>
    <script src="/webui.js"></script>
    <button id="my_button">Click me for some frontend (devtools) logs!</button>
  </html>
  "#,
    );

    wait();
}

fn log_to_js(e: Event) {
    let mut script = script(
        r#"
  console.log('Button clicked!')
  "#,
    );

    e.get_window().run_js(&mut script);
}

fn script(contents: &str) -> JavaScript {
    JavaScript {
        timeout: 0,
        script: contents.to_string(),
        error: false,
        data: "".to_string(),
    }
}
