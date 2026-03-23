// WebUI Rust - Serve a Folder Example
//
// Serves HTML files from the examples/serve_a_folder/ directory.
// Also demonstrates a custom file handler for embedded static and dynamic content.

use std::ffi::CStr;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};
use webui_rs::webui;

// Folder containing index.html / second.html (resolved at compile time)
const FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/serve_a_folder");

static MAIN_WIN: AtomicUsize = AtomicUsize::new(0);
static SECOND_WIN: AtomicUsize = AtomicUsize::new(0);
static DYNAMIC_COUNT: AtomicI32 = AtomicI32::new(0);

fn exit_app(_e: webui::Event) {
    webui::exit();
}

fn events(e: webui::Event) {
    match e.event_type {
        webui::EventType::Connected => println!("Connected."),
        webui::EventType::Disconnected => println!("Disconnected."),
        webui::EventType::MouseClick => println!("Click."),
        webui::EventType::Navigation => {
            // WebUI intercepts href clicks and sends them here when "" is bound.
            // We read the target URL and allow navigation.
            let url = e.get_string();
            println!("Navigation to: {url}");
            webui::navigate(e.window, url);
        }
        _ => {}
    }
}

fn switch_to_second_page(e: webui::Event) {
    webui::show(e.window, "second.html");
}

fn show_second_window(_e: webui::Event) {
    let win = SECOND_WIN.load(Ordering::SeqCst);
    webui::show(win, "second.html");
}

/// Custom file handler: intercepts requests before WebUI looks on disk.
/// Return `NULL` to let WebUI serve the file from the root folder instead.
unsafe extern "C" fn my_files_handler(filename: *const i8, length: *mut i32) -> *const c_void {
    let name = CStr::from_ptr(filename).to_str().unwrap_or("");
    println!("File: {name}");

    if name == "/test.txt" {
        // Static embedded response
        static RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\n\
            Content-Type: text/html\r\n\
            Content-Length: 99\r\n\r\n\
            <html>This is a static embedded file content example.\
            <script src=\"webui.js\"></script></html>";
        return RESPONSE.as_ptr() as *const c_void;
    }

    if name == "/dynamic.html" {
        // Dynamic embedded response — counter increments on each refresh
        let count = DYNAMIC_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        let body = format!(
            "<html>Dynamic file content example.<br>Count: {} \
            <a href=\"dynamic.html\">[Refresh]</a>\
            <script src=\"webui.js\"></script></html>",
            count
        );
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );

        return webui::malloc(&response, length);
    }

    // Let WebUI serve everything else from the root folder
    std::ptr::null()
}

fn main() {
    // Create all windows BEFORE binding (each new_window() resets the callback table)
    let win1 = webui::Window::new();
    let win2 = webui::Window::new();

    MAIN_WIN.store(win1.id, Ordering::SeqCst);
    SECOND_WIN.store(win2.id, Ordering::SeqCst);

    // Bindings for window 1
    win1.bind("SwitchToSecondPage", switch_to_second_page);
    win1.bind("OpenNewWindow", show_second_window);
    win1.bind("Exit", exit_app);
    win1.bind("", events); // catch-all for navigation / connect events

    // Bindings for window 2
    win2.bind("Exit", exit_app);

    // Custom file handler (intercepts /test.txt and /dynamic.html)
    win1.set_file_handler(my_files_handler);

    // Serve files from disk
    win1.set_root_folder(FOLDER);
    win2.set_root_folder(FOLDER);

    win1.set_size(800, 600);
    win1.set_position(200, 200);
    win1.show("index.html");

    webui::wait();
    webui::clean();
}
