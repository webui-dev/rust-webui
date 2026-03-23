// WebUI Rust - Virtual File System Example
//
// Files are embedded at compile time using include_str!/include_bytes!.
// At runtime, the custom file handler serves them directly from memory
// without reading anything from disk.
//
// Equivalent to the C approach of generating a vfs.h from a folder via
// a Python script and using webui_set_file_handler().

use std::ffi::CStr;
use std::os::raw::c_void;
use webui_rs::webui;
use webui_rs::webui::bindgen::webui_malloc;
use webui_rs::webui::Event;

// Embed files at compile time
const INDEX_HTML: &str = include_str!("index.html");

fn exit_app(_e: Event) {
    webui::exit();
}

/// Serve all files from the embedded virtual file system.
unsafe extern "C" fn vfs(filename: *const i8, length: *mut i32) -> *const c_void {
    let name = CStr::from_ptr(filename).to_str().unwrap_or("");

    let content: Option<(&str, &str)> = match name {
        "/" | "/index.html" => Some((INDEX_HTML, "text/html")),
        // Add more embedded files here:
        // "/style.css"  => Some((include_str!("style.css"),  "text/css")),
        // "/app.js"     => Some((include_str!("app.js"),     "application/javascript")),
        _ => None,
    };

    if let Some((body, mime)) = content {
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {mime}\r\nContent-Length: {}\r\n\r\n{body}",
            body.len()
        );

        // Allocate via WebUI — it will free the buffer automatically after serving
        let buf = webui_malloc(response.len() + 1) as *mut u8;
        std::ptr::copy_nonoverlapping(response.as_ptr(), buf, response.len());
        *buf.add(response.len()) = 0;

        if !length.is_null() {
            *length = response.len() as i32;
        }
        return buf as *const c_void;
    }

    // Return NULL to let WebUI handle unknown requests (e.g. webui.js itself)
    std::ptr::null()
}

fn main() {
    let window = webui::Window::new();

    window.bind("Exit", exit_app);
    window.set_file_handler(vfs);

    window.show("index.html");
    webui::wait();
    webui::clean();
}
