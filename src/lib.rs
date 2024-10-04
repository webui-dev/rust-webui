#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod bindgen;
pub mod events;
pub mod webui;
pub mod window;

// Re-export
// pub use bindgen::*;
// pub use events::*;
// pub use webui::*;
pub use window::Window;

use bindgen::{webui_browser, webui_config, webui_runtime};
use std::ffi::CStr;

// Browsers
pub type Browser = webui_browser;
// Runtimes
pub type Runtime = webui_runtime;
// Configs
pub type Config = webui_config;

fn char_to_string(c: *const i8) -> String {
    let cstr = unsafe { CStr::from_ptr(c) };
    let s: String = String::from_utf8_lossy(cstr.to_bytes()).to_string();
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webui_window() {
        let win = window::Window::new();
        assert_eq!(win.id, 1);
        win.show("<span>Hello World</span>");

        // Wait 2 seconds, then kill
        std::thread::sleep(std::time::Duration::from_secs(2));

        win.destroy();
    }
}
