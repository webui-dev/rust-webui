// WebUI Rust - Call JavaScript from Rust Example

use webui_rs::webui;

const HTML: &str = r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <script src="webui.js"></script>
    <title>Call JavaScript from Rust Example</title>
    <style>
      body {
        font-family: 'Arial', sans-serif;
        color: white;
        background: linear-gradient(to right, #507d91, #1c596f, #022737);
        text-align: center;
        font-size: 18px;
      }
      button, input {
        padding: 10px;
        margin: 10px;
        border-radius: 3px;
        border: 1px solid #ccc;
        box-shadow: 0 3px 5px rgba(0,0,0,0.1);
        transition: 0.2s;
      }
      button {
        background: #3498db;
        color: #fff;
        cursor: pointer;
        font-size: 16px;
      }
      h1 { text-shadow: -7px 10px 7px rgb(67 57 57 / 76%); }
      button:hover { background: #c9913d; }
      button:disabled {
        opacity: 0.6;
        cursor: not-allowed;
        filter: grayscale(30%);
      }
      button:disabled:hover { background: #3498db; }
      input:focus { outline: none; border-color: #3498db; }
    </style>
  </head>
  <body>
    <h1>WebUI - Call JavaScript from Rust</h1>
    <br>
    <h1 id="count">0</h1>
    <br>
    <button id="ManualBtn" OnClick="my_function_count();">Manual Count</button>
    <br>
    <button id="MyTest" OnClick="AutoTest();">Auto Count (Every 10ms)</button>
    <br>
    <button id="ExitBtn" OnClick="this.disabled=true; my_function_exit();">Exit</button>
    <script>
      let count = 0;
      let auto_running = false;
      function GetCount() { return count; }
      function SetCount(number) {
        document.getElementById('count').innerHTML = number;
        count = number;
      }
      function AutoTest() {
        if (auto_running) return;
        auto_running = true;
        document.getElementById('MyTest').disabled = true;
        document.getElementById('ManualBtn').disabled = true;
        setInterval(function(){ my_function_count(); }, 10);
      }
    </script>
  </body>
</html>"#;

fn my_function_exit(_e: webui::Event) {
    webui::exit();
}

fn my_function_count(e: webui::Event) {
    let window = e.get_window();

    // Run JS and capture return value.
    // The second argument is the response buffer size in bytes (0 = default 8KB).
    let js = window.run_js("return GetCount();", 1024);
    if js.error {
        if !window.is_shown() {
            println!("Window closed.");
        } else {
            println!("JavaScript Error: {}", js.data);
        }
        return;
    }

    println!("JavaScript returned: {}", js.data);

    // Parse count, increment, push back
    let count: i64 = js.data.trim().parse().unwrap_or(0);
    let next = count + 1;

    // Run JS without capturing return value (faster)
    // No buffer needed since this is a fire and forget call.
    window.run(&format!("SetCount({next});"));
}

fn main() {
    // Process UI events one at a time so count stays consistent
    webui::set_config(webui::Config::UiEventBlocking, true);

    let window = webui::Window::new();
    window.bind("my_function_count", my_function_count);
    window.bind("my_function_exit", my_function_exit);

    window.show(HTML);
    webui::wait();
    webui::clean();
}
