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
#![allow(unsafe_code)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod bindgen;

// Modules
use std::collections::HashMap;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

use bindgen::*;
use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};

// Consts
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub type size_t = ::std::os::raw::c_ulong;
pub type wchar_t = ::std::os::raw::c_int;

// Browsers
pub enum WebUIBrowser {
    NoBrowser = 0,
    AnyBrowser = 1,
    Chrome,
    Firefox,
    Edge,
    Safari,
    Chromium,
    Opera,
    Brave,
    Vivaldi,
    Epic,
    Yandex,
    ChromiumBased,
}

impl Clone for WebUIBrowser {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for WebUIBrowser {}

impl WebUIBrowser {
    pub fn to_usize(&self) -> usize {
        *self as usize
    }
}

// Impl equality operator
impl PartialEq for WebUIBrowser {
  fn eq(&self, other: &Self) -> bool {
      self.to_usize() == other.to_usize()
  }
}

// Runtimes
pub enum WebUIRuntime {
    None = 0,
    Deno = 1,
    NodeJS = 2,
}

// Events
pub enum WebUIEvent {
    WebUiEventDisconnected = 0,
    WebUiEventConnected = 1,
    WebUiEventMultiConnection = 2,
    WebUiEventUnwantedConnection = 3,
    WebUiEventMouseClick = 4,
    WebUiEventNavigation = 5,
    WebUiEventCallback = 6,
}

// Implement into<usize>
impl WebUIEvent {
    pub fn from_usize(value: usize) -> WebUIEvent {
        match value {
            0 => WebUIEvent::WebUiEventDisconnected,
            1 => WebUIEvent::WebUiEventConnected,
            2 => WebUIEvent::WebUiEventMultiConnection,
            3 => WebUIEvent::WebUiEventUnwantedConnection,
            4 => WebUIEvent::WebUiEventMouseClick,
            5 => WebUIEvent::WebUiEventNavigation,
            6 => WebUIEvent::WebUiEventCallback,
            _ => WebUIEvent::WebUiEventCallback,
        }
    }
}

pub struct JavaScript {
    pub timeout: usize,
    pub script: String,
    pub error: bool,
    pub data: String,
}

// Window, EventType, Element, EventNumber, BindID
pub struct Event {
    pub window: usize,
    pub event_type: WebUIEvent,
    pub element: *mut c_char,
    pub event_number: usize,
    pub bind_id: usize,
}

impl Event {
    pub fn get_window(&self) -> Window {
        Window::from_id(self.window)
    }
}

pub struct Window {
    pub id: usize,
}

impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}

impl Window {
    pub fn new() -> Window {
        let id = new_window();
        Window { id }
    }

    pub fn from_id(id: usize) -> Window {
        Window { id }
    }

    pub fn show(&self, content: impl AsRef<str>) -> bool {
        show(self.id, content.as_ref())
    }

    pub fn show_browser(&self, content: impl AsRef<str>, browser: WebUIBrowser) -> bool {
        show_browser(self.id, content.as_ref(), browser)
    }

    pub fn is_shown(&self) -> bool {
        is_shown(self.id)
    }

    pub fn bind(&self, element: impl AsRef<str>, func: fn(Event)) {
        bind(self.id, element.as_ref(), func);
    }

    pub fn run_js(&self, js: impl AsRef<str>) -> JavaScript {
        let mut js = JavaScript {
            timeout: 0,
            script: js.as_ref().to_string(),
            error: false,
            data: "".to_string(),
        };

        run_js(self.id, &mut js);

        js
    }

    pub fn close(&self) {
        close(self.id);
    }

    pub fn destroy(&self) {
        destroy(self.id);
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        destroy(self.id);
    }
}

// List of Rust user functions (2-dimensional array)
// static mut func_list: [[Option::<fn(e: Event) -> ()>; 64]; 64] = [[64; 64]; 64];
// static mut func_array: Vec<Vec<fn(Event)>> = vec![vec![]; 1024];
// static mut elements_map = HashMap::<String, usize>::new();
// static mut elements_map: HashMap::new();

type FunctionType = fn(Event);
const ROWS: usize = 64;
const COLS: usize = 64;

#[derive(Copy, Clone, Default)]
enum GlobalArray {
    #[default]
    None,
    Some(FunctionType),
}

static mut GLOBAL_ARRAY: [[GlobalArray; COLS]; ROWS] = [[GlobalArray::None; COLS]; ROWS];

lazy_static! {
    static ref ELEMENTS_MAP: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
    // static mut func_array: Vec<Vec<fn(Event)>> = vec![vec![]; 1024];
}

// Save a string in the map and return its index
fn save_string(mut map: MutexGuard<HashMap<String, usize>>, s: &str) -> usize {
    // Check if the string already exists in the map
    if let Some(&index) = map.get(s) {
        return index;
    }

    // If the string does not exist, add it to the map and return the new index
    let index = map.len();
    map.insert(s.to_owned(), index);
    index
}

// Search for a string in the map and return its index if found, or -1 if not found
fn find_string(map: &HashMap<String, usize>, s: &str) -> isize {
    if let Some(&index) = map.get(s) {
        index as isize
    } else {
        -1
    }
}

fn char_to_string(c: *const i8) -> String {
    let cstr = unsafe { CStr::from_ptr(c) };
    let s: String = String::from_utf8_lossy(cstr.to_bytes()).to_string();
    s
}

fn cstr_to_string(c: CString) -> String {
    let s: String = String::from_utf8_lossy(c.to_bytes()).to_string();
    s
}

pub fn run_js(win: usize, js: &mut JavaScript) {
    /// The WebUI Script Interface
    struct WebUIScriptIntf {
        timeout: usize,
        script: *mut i8,
        error: bool,
        data: *const i8,
        length: usize,
    }

    unsafe {
        // Script String to i8/u8
        let script_cpy = js.script.clone();
        let script_c_str = CString::new(script_cpy).unwrap();
        let script_c_char: *mut c_char = script_c_str.as_ptr() as *mut c_char;

        let wuisi = WebUIScriptIntf {
            timeout: js.timeout,
            script: script_c_char,
            data: script_c_char,
            error: false,
            length: 0,
        };

        webui_script(
            win,
            wuisi.script,
            wuisi.timeout,
            script_c_char,
            wuisi.length,
        );

        js.error = wuisi.error;
        js.data = char_to_string(wuisi.data);
    }
}

pub fn new_window() -> usize {
    unsafe {
        GLOBAL_ARRAY = [[GlobalArray::None; COLS]; ROWS];
        webui_new_window()
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

pub fn show(win: usize, content: impl AsRef<str> + Into<Vec<u8>>) -> bool {
    unsafe {
        // Content String to i8/u8
        let content_c_str = CString::new(content).unwrap();
        let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

        webui_show(win, content_c_char)
    }
}

pub fn show_browser(
    win: usize,
    content: impl AsRef<str> + Into<Vec<u8>>,
    browser: WebUIBrowser,
) -> bool {
    let content_c_str = CString::new(content).unwrap();
    let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

    unsafe { webui_show_browser(win, content_c_char, browser as usize) }
}

pub fn is_shown(win: usize) -> bool {
    unsafe { webui_is_shown(win) }
}

pub fn close(win: usize) {
    unsafe {
        webui_close(win);
    }
}

pub fn destroy(win: usize) {
    unsafe {
        webui_destroy(win);
    }
}

unsafe extern "C" fn events_handler(
    window: usize,
    event_type: usize,
    element: *mut ::std::os::raw::c_char,
    event_number: usize,
    bind_id: usize,
) {
    let map = ELEMENTS_MAP.lock().unwrap();

    let element_index = find_string(&map, &char_to_string(element));
    if element_index < 0 {
        return;
    }

    let evt = Event {
        window,
        event_type: WebUIEvent::from_usize(event_type),
        element,
        event_number,
        bind_id,
    };

    // Call the Rust user function
    let element_index_64 = element_index as usize;
    unsafe {
        let window_id = webui_interface_get_window_id(window);
        let window_id_64 = window_id;
        // func_list[window_id_64][element_index_64].expect("non-null function pointer")(E);
        // func_array[window_id_64][element_index_64](E);
        // if let Some(func) = GLOBAL_ARRAY[window_id_64][element_index_64] {
        //     func(E.clone());
        // }
        if let GlobalArray::Some(func) = GLOBAL_ARRAY[window_id_64][element_index_64] {
            func(evt);
        }
    }
}

pub fn bind(win: usize, element: &str, func: fn(Event)) {
    let map = ELEMENTS_MAP.lock().unwrap();

    // Element String to i8/u8
    let element_c_str = CString::new(element).unwrap();
    let element_c_char: *const c_char = element_c_str.as_ptr() as *const c_char;

    let element_index = save_string(map, element);

    // Bind
    unsafe {
        let f: Option<
            unsafe extern "C" fn(usize, usize, *mut ::std::os::raw::c_char, usize, usize),
        > = Some(events_handler);

        let window_id = webui_interface_get_window_id(win);
        let window_id_64 = window_id;
        let element_index_64 = element_index;

        webui_interface_bind(win, element_c_char, f);

        // Add the Rust user function to the list
        // let user_cb: Option<fn(e: Event)> = Some(func);
        // func_list[window_id_64][element_index_64] = user_cb;
        // func_array[window_id_64][element_index_64] = func;
        // GLOBAL_ARRAY[window_id_64][element_index_64] = Some(func as FunctionType);

        GLOBAL_ARRAY[window_id_64][element_index_64] = GlobalArray::Some(func as FunctionType);
    }
}
