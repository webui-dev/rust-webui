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
#![allow(non_camel_case_types)]
#![allow(unsafe_code)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(non_upper_case_globals)]

mod bindgen;

// Modules
use std::os::raw::c_char;
use std::ffi::CString;
use std::ffi::CStr;
use std::collections::HashMap;

use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};
use bindgen::*;

// Consts
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub type size_t = ::std::os::raw::c_ulong;
pub type wchar_t = ::std::os::raw::c_int;

// Browsers
pub const AnyBrowser: u32 = 0;
pub const Chrome: u32 = 1;
pub const Firefox: u32 = 2;
pub const Edge: u32 = 3;
pub const Safari: u32 = 4;
pub const Chromium: u32 = 5;
pub const Opera: u32 = 6;
pub const Brave: u32 = 7;
pub const Vivaldi: u32 = 8;
pub const Epic: u32 = 9;
pub const Yandex: u32 = 10;

// Runtimes
pub const None: u32 = 0;
pub const Deno: u32 = 1;
pub const NodeJS: u32 = 2;

// Events
pub const WEBUI_EVENT_DISCONNECTED: u32 = 0;
pub const WEBUI_EVENT_CONNECTED: u32 = 1;
pub const WEBUI_EVENT_MULTI_CONNECTION: u32 = 2;
pub const WEBUI_EVENT_UNWANTED_CONNECTION: u32 = 3;
pub const WEBUI_EVENT_MOUSE_CLICK: u32 = 4;
pub const WEBUI_EVENT_NAVIGATION: u32 = 5;
pub const WEBUI_EVENT_CALLBACK: u32 = 6;

pub struct JavaScript {
    pub timeout: usize,
    pub script: String,
    pub error: bool,
    pub data: String,
}

// Window, EventType, Element, EventNumber, BindID
pub struct Event {
    pub window: usize,
    pub event_type: usize,
    pub element: *mut c_char,
    pub event_number: usize,
    pub bind_id: usize,
}

// List of Rust user functions (2-dimensional array)
// static mut func_list: [[Option::<fn(e: Event) -> ()>; 64]; 64] = [[64; 64]; 64];
// static mut func_array: Vec<Vec<fn(Event)>> = vec![vec![]; 1024];
// static mut elements_map = HashMap::<String, usize>::new();
// static mut elements_map: HashMap::new();

type FunctionType = fn(Event);
const ROWS: usize = 64;
const COLS: usize = 64;

#[derive(Copy, Clone)]
enum GlobalArray {
    None,
    Some(FunctionType),
}

impl Default for GlobalArray {
    fn default() -> Self {
        GlobalArray::None
    }
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
    return s;
}

fn cstr_to_string(c: CString) -> String {
    let s: String = String::from_utf8_lossy(c.to_bytes()).to_string();
    return s;
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
        let script_cpy = String::from(js.script.clone());
        let script_c_str = CString::new(script_cpy).unwrap();
        let script_c_char: *mut c_char = script_c_str.as_ptr() as *mut c_char;

        let wuisi = WebUIScriptIntf {
            timeout: js.timeout,
            script: script_c_char as *mut i8,
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
        return webui_new_window();
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

pub fn show(win: usize, content: &str) -> bool {
    unsafe {
        // Content String to i8/u8
        let content_c_str = CString::new(content).unwrap();
        let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

        return webui_show(win, content_c_char);
    }
}

unsafe extern "C" fn events_handler(
    window: usize,
    event_type: usize,
    element: *mut ::std::os::raw::c_char,
    event_number: usize,
    bind_id: usize,
) {
    let mut map = ELEMENTS_MAP.lock().unwrap();

    let element_index = find_string(&mut map, &char_to_string(element));
    if element_index < 0 {
        return;
    }

    let evt = Event {
        window: window,
        event_type: event_type,
        element: element,
        event_number: event_number,
        bind_id: bind_id,
    };

    // Call the Rust user function
    let element_index_64 = element_index as usize;
    unsafe {
        let window_id = webui_interface_get_window_id(window);
        let window_id_64 = window_id as usize;
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

// pub fn bind(win: usize, element: &str, func: fn(Event)) {
//     let map = ELEMENTS_MAP.lock().unwrap();

//     // Element String to i8/u8
//     let element_c_str = CString::new(element).unwrap();
//     let element_c_char: *const c_char = element_c_str.as_ptr() as *const c_char;

//     let element_index = save_string(map, element);

//     // Bind
//     unsafe {
//         let f = Some(events_handler);

//         let window_id = webui_interface_get_window_id(win);
//         let window_id_64 = window_id as usize;
//         let element_index_64 = element_index as usize;

//         webui_interface_bind(win, element_c_char, f);

//         // Add the Rust user function to the list
//         // let user_cb: Option<fn(e: Event)> = Some(func);
//         // func_list[window_id_64][element_index_64] = user_cb;
//         // func_array[window_id_64][element_index_64] = func;
//         // GLOBAL_ARRAY[window_id_64][element_index_64] = Some(func as FunctionType);

//         GLOBAL_ARRAY[window_id_64][element_index_64] = GlobalArray::Some(func as FunctionType);
//     }
// }
