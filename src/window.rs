// Flags
// #![allow(unsafe_code)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// Modules
use crate::char_to_string;
use crate::events::Event;
use crate::webui::BindStore;
use crate::webui::*;
use crate::Browser;
use crate::Runtime;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::LazyLock;

use crate::bindgen::*;

const WINDOWS: usize = 64;
const ELEMENTS: usize = 64;
static mut BIND_STORE: LazyLock<BindStore<fn(Event)>> = LazyLock::new(|| BindStore::new());

pub struct Window {
    pub id: usize,
}

impl Window {
    pub fn new() -> Window {
        let id = new_window();
        Window { id }
    }

    pub fn from_id(id: usize) -> Window {
        Window { id }
    }

    pub fn bind(&self, element: &str, func: fn(Event)) -> usize {
        // Element String to i8/u8
        let element_c_str = CString::new(element).unwrap();
        let element_c_char: *const c_char = element_c_str.as_ptr() as *const c_char;

        // Bind
        unsafe {
            let f: Option<unsafe extern "C" fn(*mut webui_event_t)> = Some(bind_events_handler);

            let window_id = webui_interface_get_window_id(self.id);

            // Add the Rust user function to the list
            BIND_STORE.add_function(window_id, element, func);

            webui_bind(self.id, element_c_char, f)
        }
    }

    pub fn get_best_browser(&self) -> Browser {
        unsafe {
            match webui_get_best_browser(self.id) {
                0 => Browser::NoBrowser,
                1 => Browser::AnyBrowser,
                2 => Browser::Chrome,
                3 => Browser::Firefox,
                4 => Browser::Edge,
                5 => Browser::Safari,
                6 => Browser::Chromium,
                7 => Browser::Opera,
                8 => Browser::Brave,
                9 => Browser::Vivaldi,
                10 => Browser::Epic,
                11 => Browser::Yandex,
                12 => Browser::ChromiumBased,
                13 => Browser::Webview,
                _ => Browser::NoBrowser,
            }
        }
    }

    pub fn show(&self, content: &str) -> bool {
        unsafe {
            // Content String to i8/u8
            let content_c_str = CString::new(content).unwrap();
            let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

            webui_show(self.id, content_c_char)
        }
    }

    pub fn show_browser(&self, content: &str, browser: Browser) -> bool {
        let content_c_str = CString::new(content).unwrap();
        let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

        unsafe { webui_show_browser(self.id, content_c_char, browser as usize) }
    }

    pub fn start_server(&self, content: &str) -> String {
        let content_c_str = CString::new(content).unwrap();
        let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

        unsafe {
            let server = webui_start_server(self.id, content_c_char);
            char_to_string(server)
        }
    }

    pub fn show_wv(&self, content: &str) -> bool {
        let content_c_str = CString::new(content).unwrap();
        let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

        unsafe { webui_show_wv(self.id, content_c_char) }
    }

    pub fn set_kiosk(&self, status: bool) {
        unsafe {
            webui_set_kiosk(self.id, status);
        }
    }

    pub fn set_high_contrast(&self, status: bool) {
        unsafe {
            webui_set_high_contrast(self.id, status);
        }
    }

    pub fn close(self) {
        unsafe {
            webui_close(self.id);
        }
    }

    pub fn destroy(&self) {
        unsafe {
            webui_destroy(self.id);
        }
    }

    pub fn set_root_folder(&self, folder: &str) {
        let folder_c_str = CString::new(folder).unwrap();
        let folder_c_char: *const c_char = folder_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_set_root_folder(self.id, folder_c_char);
        }
    }

    pub fn set_file_handler(
        &self,
        handler: unsafe extern "C" fn(*const i8, *mut i32) -> *const std::os::raw::c_void,
    ) {
        unsafe {
            webui_set_file_handler(self.id, Some(handler));
        }
    }

    pub fn is_shown(&self) -> bool {
        unsafe { webui_is_shown(self.id) }
    }

    pub fn set_icon(&self, icon: &str, kind: &str) {
        let icon_c_str = CString::new(icon).unwrap();
        let kind_c_str = CString::new(kind).unwrap();
        let icon_c_char: *const c_char = icon_c_str.as_ptr() as *const c_char;
        let kind_c_char: *const c_char = kind_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_set_icon(self.id, icon_c_char, kind_c_char);
        }
    }

    pub fn send_raw(&self, function: &str, data: &[u8]) {
        let size = data.len();
        let raw = data.as_ptr() as *mut std::os::raw::c_void;
        let function_c_str = CString::new(function).unwrap();
        let function_c_char: *const c_char = function_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_send_raw(self.id, function_c_char, raw, size);
        }
    }

    pub fn set_hide(&self, status: bool) {
        unsafe {
            webui_set_hide(self.id, status);
        }
    }

    pub fn set_size(&self, width: u32, height: u32) {
        unsafe {
            webui_set_size(self.id, width, height);
        }
    }

    pub fn set_position(&self, x: u32, y: u32) {
        unsafe {
            webui_set_position(self.id, x, y);
        }
    }

    pub fn set_profile(&self, name: &str, path: &str) {
        let name_c_str = CString::new(name).unwrap();
        let path_c_str = CString::new(path).unwrap();
        let name_c_char: *const c_char = name_c_str.as_ptr() as *const c_char;
        let path_c_char: *const c_char = path_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_set_profile(self.id, name_c_char, path_c_char);
        }
    }

    pub fn set_proxy(&self, proxy: &str) {
        let proxy_c_str = CString::new(proxy).unwrap();
        let proxy_c_char: *const c_char = proxy_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_set_proxy(self.id, proxy_c_char);
        }
    }

    pub fn get_url(&self) -> String {
        unsafe {
            let url = webui_get_url(self.id);
            char_to_string(url)
        }
    }

    pub fn set_public(&self, status: bool) {
        unsafe {
            webui_set_public(self.id, status);
        }
    }

    pub fn navigate(&self, url: &str) {
        let url_c_str = CString::new(url).unwrap();
        let url_c_char: *const c_char = url_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_navigate(self.id, url_c_char);
        }
    }

    pub fn delete_profile(&self) {
        unsafe {
            webui_delete_profile(self.id);
        }
    }

    pub fn get_parent_process_id(&self) -> usize {
        unsafe { webui_get_parent_process_id(self.id) }
    }

    pub fn get_child_process_id(&self) -> usize {
        unsafe { webui_get_child_process_id(self.id) }
    }

    pub fn get_port(&self) -> usize {
        unsafe { webui_get_port(self.id) }
    }

    pub fn set_port(&self, port: usize) -> bool {
        unsafe { webui_set_port(self.id, port) }
    }

    pub fn set_event_blocking(&self, status: bool) {
        unsafe {
            webui_set_event_blocking(self.id, status);
        }
    }

    pub fn run(&self, script: &str) {
        let script_c_str = CString::new(script).unwrap();
        let script_c_char: *const c_char = script_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_run(self.id, script_c_char);
        }
    }

    pub fn script(&self, script: &str, timeout: usize, buffer_length: usize) -> Result<String, ()> {
        let script_c_str = CString::new(script).unwrap();
        let script_c_char: *const c_char = script_c_str.as_ptr() as *const c_char;

        let buffer_c_str = CString::new(vec![0; buffer_length]).unwrap();
        let buffer_c_char: *mut c_char = buffer_c_str.as_ptr() as *mut c_char;

        unsafe {
            match webui_script(
                self.id,
                script_c_char,
                timeout,
                buffer_c_char,
                buffer_length,
            ) {
                true => Ok(char_to_string(buffer_c_char)),
                false => Err(()),
            }
        }
    }

    pub fn set_runtime(&self, runtime: Runtime) {
        unsafe {
            webui_set_runtime(self.id, runtime as usize);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.destroy();
    }
}

unsafe extern "C" fn bind_events_handler(event: *mut webui_event_t) {
    let evt = Event::new(event);

    // Call the Rust user function
    unsafe {
        let window_id = webui_interface_get_window_id((*event).window);

        if let Some(func) = BIND_STORE.get_function(window_id, &evt.element) {
            func(evt);
        }
    }
}
