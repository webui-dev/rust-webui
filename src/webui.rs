/*
  WebUI Library 2.2.0
  http://_webui_core.me
  https://github.com/alifcommunity/webui
  Copyright (c) 2020-2023 Hassan Draga.
  Licensed under GNU General Public License v2.0.
  All rights reserved.
  Canada.
*/

// Flags
// #![allow(unsafe_code)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// Modules
use crate::char_to_string;
use crate::events::EventSimple;
use crate::events::EventType;
use crate::Browser;
use crate::Config;
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::LazyLock;
use std::sync::Mutex;

use crate::bindgen::*;

const WINDOWS: usize = 64;
const ELEMENTS: usize = 64;

pub struct BindStore<T> {
    func_store: Mutex<[[Option<T>; ELEMENTS]; WINDOWS]>,
    elements_map: Mutex<HashMap<String, usize>>,
}

impl<T: Copy> BindStore<T> {
    pub fn new() -> BindStore<T> {
        BindStore {
            func_store: Mutex::new([[None; ELEMENTS]; WINDOWS]),
            elements_map: Mutex::new(HashMap::new()),
        }
    }

    fn save_string(&self, s: &str) -> usize {
        let mut map = self.elements_map.lock().unwrap();
        // Check if the string already exists in the map
        if let Some(&index) = map.get(s) {
            return index;
        }

        // If the string does not exist, add it to the map and return the new index
        let index = map.len();
        map.insert(s.to_owned(), index);
        index
    }

    fn find_string(&self, s: &str) -> isize {
        let map = self.elements_map.lock().unwrap();
        if let Some(&index) = map.get(s) {
            index as isize
        } else {
            -1
        }
    }

    pub fn add_function(&self, window: usize, element: &str, func: T) {
        let element_index = self.save_string(element);
        self.func_store.lock().unwrap()[window][element_index] = Some(func);
    }

    pub fn get_function(&self, window: usize, element: &str) -> Option<T> {
        let element_index = self.find_string(element);
        if element_index < 0 {
            return None;
        }
        self.func_store.lock().unwrap()[window][element_index as usize]
    }
}

static mut BIND_STORE_SIMPLE: LazyLock<BindStore<fn(EventSimple)>> =
    LazyLock::new(|| BindStore::new());

// Function Implementations
pub fn new_window() -> usize {
    unsafe {
        // GLOBAL_ARRAY = [[GlobalArray::None; COLS]; ROWS];
        webui_new_window()
    }
}

pub fn new_window_id(id: usize) -> usize {
    unsafe {
        // GLOBAL_ARRAY = [[GlobalArray::None; COLS]; ROWS];
        webui_new_window_id(id)
    }
}

pub fn get_new_window_id() -> usize {
    unsafe { webui_get_new_window_id() }
}

pub fn is_high_contrast() -> bool {
    unsafe { webui_is_high_contrast() }
}

pub fn browser_exist(browser: Browser) {
    unsafe {
        webui_browser_exist(browser as usize);
    }
}

pub fn wait() {
    unsafe {
        webui_wait();
    }
}

pub fn exit() {
    unsafe {
        webui_exit();
    }
}

pub fn set_default_root_folder(folder: &str) {
    let folder_c_str = CString::new(folder).unwrap();
    let folder_c_char: *const c_char = folder_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_set_default_root_folder(folder_c_char);
    }
}

pub fn set_timeout(seconds: usize) {
    unsafe {
        webui_set_timeout(seconds);
    }
}

pub fn encode(data: &str) -> String {
    let data_c_str = CString::new(data).unwrap();
    let data_c_char: *const c_char = data_c_str.as_ptr() as *const c_char;

    unsafe {
        let encoded = webui_encode(data_c_char);
        char_to_string(encoded)
    }
}

pub fn decode(data: &str) -> String {
    let data_c_str = CString::new(data).unwrap();
    let data_c_char: *const c_char = data_c_str.as_ptr() as *const c_char;

    unsafe {
        let decoded = webui_decode(data_c_char);
        char_to_string(decoded)
    }
}

pub fn free(data: *mut std::os::raw::c_void) {
    unsafe {
        webui_free(data);
    }
}

pub fn malloc(size: usize) -> *mut std::os::raw::c_void {
    unsafe { webui_malloc(size) }
}

pub fn open_url(url: &str) {
    let url_c_str = CString::new(url).unwrap();
    let url_c_char: *const c_char = url_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_open_url(url_c_char);
    }
}

pub fn clean() {
    unsafe {
        webui_clean();
    }
}

pub fn delete_all_profiles() {
    unsafe {
        webui_delete_all_profiles();
    }
}

pub fn get_free_port() -> usize {
    unsafe { webui_get_free_port() }
}

pub fn set_config(option: Config, enabled: bool) {
    unsafe {
        webui_set_config(option as webui_config, enabled);
    }
}

pub fn get_mime_type(file: &str) -> String {
    let file_c_str = CString::new(file).unwrap();
    let file_c_char: *const c_char = file_c_str.as_ptr() as *const c_char;

    unsafe {
        let mime = webui_get_mime_type(file_c_char);
        char_to_string(mime)
    }
}

pub fn set_tls_certificate(cert_pem: &str, key_pem: &str) -> bool {
    let cert_pem_c_str = CString::new(cert_pem).unwrap();
    let key_pem_c_str = CString::new(key_pem).unwrap();
    let cert_pem_c_char: *const c_char = cert_pem_c_str.as_ptr() as *const c_char;
    let key_pem_c_char: *const c_char = key_pem_c_str.as_ptr() as *const c_char;

    unsafe { webui_set_tls_certificate(cert_pem_c_char, key_pem_c_char) }
}

unsafe extern "C" fn events_handler(
    window: usize,
    event_type: usize,
    element: *mut ::std::os::raw::c_char,
    event_number: usize,
    bind_id: usize,
) {
    // Call the Rust user function
    unsafe {
        let window_id = webui_interface_get_window_id(window);

        if let Some(func) = BIND_STORE_SIMPLE.get_function(window_id, &char_to_string(element)) {
            let evt = EventSimple {
                win: window,
                event_type: EventType::from_usize(event_type),
                element: char_to_string(element),
                event_number,
                bind_id,
            };

            func(evt);
        }
    }
}

pub fn interface_bind(win: usize, element: &str, func: fn(EventSimple)) -> usize {
    // Element String to i8/u8
    let element_c_str = CString::new(element).unwrap();
    let element_c_char: *const c_char = element_c_str.as_ptr() as *const c_char;

    // Bind
    unsafe {
        let f: Option<
            unsafe extern "C" fn(usize, usize, *mut ::std::os::raw::c_char, usize, usize),
        > = Some(events_handler);

        let window_id = webui_interface_get_window_id(win);

        // Add the Rust user function to the list
        BIND_STORE_SIMPLE.add_function(window_id, element, func);

        webui_interface_bind(win, element_c_char, f)
    }
}

pub fn interface_is_app_running() -> bool {
    unsafe { webui_interface_is_app_running() }
}

pub fn interface_get_window_id(win: usize) -> usize {
    unsafe { webui_interface_get_window_id(win) }
}
