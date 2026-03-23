// WebUI Rust - Call Rust from JavaScript Example

use webui_rs::webui;
use webui_rs::webui::Event;

const HTML: &str = r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <script src="webui.js"></script>
    <title>Call Rust from JavaScript Example</title>
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
      input:focus { outline: none; border-color: #3498db; }
    </style>
  </head>
  <body>
    <h1>WebUI - Call Rust from JavaScript</h1>
    <p>Call Rust functions with arguments (<em>See the logs in your terminal</em>)</p>
    <button onclick="my_function_string('Hello', 'World');">Call my_function_string()</button>
    <br>
    <button onclick="my_function_integer(123, 456, 789, 12345.6789);">Call my_function_integer()</button>
    <br>
    <button onclick="my_function_boolean(true, false);">Call my_function_boolean()</button>
    <br>
    <button onclick="my_function_raw_binary(new Uint8Array([0x41,0x42,0x43]), big_arr);">
      Call my_function_raw_binary()</button>
    <br>
    <p>Call a Rust function that returns a response</p>
    <button onclick="MyJS();">Call my_function_with_response()</button>
    <div>Double: <input type="text" id="MyInputID" value="2"></div>
    <script>
      const arr_size = 512 * 1000;
      const big_arr = new Uint8Array(arr_size);
      big_arr[0] = 0xA1;
      big_arr[arr_size - 1] = 0xA2;
      function MyJS() {
        const MyInput = document.getElementById('MyInputID');
        const number = MyInput.value;
        my_function_with_response(number, 2).then((response) => {
          MyInput.value = response;
        });
      }
    </script>
  </body>
</html>"#;

fn my_function_string(e: Event) {
    // JavaScript: my_function_string('Hello', 'World')
    println!("my_function_string 1: {}", e.get_string());    // Hello
    println!("my_function_string 2: {}", e.get_string_at(1)); // World
}

fn my_function_integer(e: Event) {
    // JavaScript: my_function_integer(123, 456, 789, 12345.6789)
    println!("my_function_integer: {} arguments", e.get_count());
    println!("my_function_integer 1: {}", e.get_int());        // 123
    println!("my_function_integer 2: {}", e.get_int_at(1));    // 456
    println!("my_function_integer 3: {}", e.get_int_at(2));    // 789
    println!("my_function_integer 4: {}", e.get_float_at(3));  // 12345.6789
}

fn my_function_boolean(e: Event) {
    // JavaScript: my_function_boolean(true, false)
    println!("my_function_boolean 1: {}", e.get_bool());       // true
    println!("my_function_boolean 2: {}", e.get_bool_at(1));   // false
}

fn my_function_raw_binary(e: Event) {
    // JavaScript: my_function_raw_binary(new Uint8Array([0x41,0x42,0x43]), big_arr)
    let bytes1 = e.get_bytes();
    let bytes2 = e.get_bytes_at(1);

    print!("my_function_raw_binary 1 ({} bytes): ", bytes1.len());
    for b in &bytes1 {
        print!("0x{b:02x} ");
    }
    println!();

    let valid = bytes2.first() == Some(&0xA1) && bytes2.last() == Some(&0xA2);
    println!("my_function_raw_binary 2 big ({} bytes): valid? {valid}", bytes2.len());
}

fn my_function_with_response(e: Event) {
    // JavaScript: my_function_with_response(number, 2).then(...)
    let number = e.get_int();
    let times = e.get_int_at(1);
    let res = number * times;
    println!("my_function_with_response: {number} * {times} = {res}");
    e.return_int(res);
}

fn main() {
    let window = webui::Window::new();

    window.bind("my_function_string", my_function_string);
    window.bind("my_function_integer", my_function_integer);
    window.bind("my_function_boolean", my_function_boolean);
    window.bind("my_function_raw_binary", my_function_raw_binary);
    window.bind("my_function_with_response", my_function_with_response);

    window.show(HTML);
    webui::wait();
    webui::clean();
}
