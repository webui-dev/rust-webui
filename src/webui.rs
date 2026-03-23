/*
  WebUI Library
  http://webui.me
  https://github.com/webui-dev/rust-webui
  Copyright (c) 2020-2026 Hassan Draga.
  Licensed under MIT License.
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
    Webview,
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
    Bun = 3,
}

// Events
pub enum WebUIEvent {
    WebUiEventDisconnected = 0,
    WebUiEventConnected = 1,
    WebUiEventMouseClick = 2,
    WebUiEventNavigation = 3,
    WebUiEventCallback = 4,
}

impl WebUIEvent {
    pub fn from_usize(value: usize) -> WebUIEvent {
        match value {
            0 => WebUIEvent::WebUiEventDisconnected,
            1 => WebUIEvent::WebUiEventConnected,
            2 => WebUIEvent::WebUiEventMouseClick,
            3 => WebUIEvent::WebUiEventNavigation,
            4 => WebUIEvent::WebUiEventCallback,
            _ => WebUIEvent::WebUiEventCallback,
        }
    }
}

// Config options
pub enum WebUIConfig {
    ShowWaitConnection = 0,
    UiEventBlocking = 1,
    FolderMonitor = 2,
    MultiClient = 3,
    UseCookies = 4,
    AsynchronousResponse = 5,
}

impl WebUIConfig {
    pub fn to_usize(&self) -> usize {
        match self {
            WebUIConfig::ShowWaitConnection => 0,
            WebUIConfig::UiEventBlocking => 1,
            WebUIConfig::FolderMonitor => 2,
            WebUIConfig::MultiClient => 3,
            WebUIConfig::UseCookies => 4,
            WebUIConfig::AsynchronousResponse => 5,
        }
    }
}

// Logger levels
pub enum WebUILoggerLevel {
    Debug = 0,
    Info = 1,
    Error = 2,
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

    pub fn show_client(&self, content: impl AsRef<str>) -> bool {
        let content_c_str = CString::new(content.as_ref()).unwrap();
        let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;
        unsafe { webui_interface_show_client(self.window, self.event_number, content_c_char) }
    }

    pub fn close_client(&self) {
        unsafe { webui_interface_close_client(self.window, self.event_number) }
    }

    pub fn send_raw_client(&self, func: impl AsRef<str>, raw: &[u8]) {
        let func_c_str = CString::new(func.as_ref()).unwrap();
        let func_c_char: *const c_char = func_c_str.as_ptr() as *const c_char;
        unsafe {
            webui_interface_send_raw_client(
                self.window,
                self.event_number,
                func_c_char,
                raw.as_ptr() as *const std::os::raw::c_void,
                raw.len(),
            );
        }
    }

    pub fn navigate_client(&self, url: impl AsRef<str>) {
        let url_c_str = CString::new(url.as_ref()).unwrap();
        let url_c_char: *const c_char = url_c_str.as_ptr() as *const c_char;
        unsafe { webui_interface_navigate_client(self.window, self.event_number, url_c_char) }
    }

    pub fn run_client(&self, script: impl AsRef<str>) {
        let script_c_str = CString::new(script.as_ref()).unwrap();
        let script_c_char: *const c_char = script_c_str.as_ptr() as *const c_char;
        unsafe { webui_interface_run_client(self.window, self.event_number, script_c_char) }
    }

    pub fn script_client(&self, js: impl AsRef<str>) -> JavaScript {
        let mut js_obj = JavaScript {
            timeout: 0,
            script: js.as_ref().to_string(),
            error: false,
            data: "".to_string(),
        };

        let script_c_str = CString::new(js_obj.script.clone()).unwrap();
        let script_c_char: *const c_char = script_c_str.as_ptr() as *const c_char;

        const BUFFER_SIZE: usize = 1024 * 8;
        let mut buffer = vec![0i8; BUFFER_SIZE];

        let result = unsafe {
            webui_interface_script_client(
                self.window,
                self.event_number,
                script_c_char,
                js_obj.timeout,
                buffer.as_mut_ptr(),
                BUFFER_SIZE,
            )
        };

        js_obj.error = !result;
        js_obj.data = unsafe {
            CStr::from_ptr(buffer.as_ptr())
                .to_string_lossy()
                .into_owned()
        };

        js_obj
    }

    fn as_raw(&self) -> webui_event_t {
        webui_event_t {
            window: self.window,
            event_type: 0,
            element: self.element,
            event_number: self.event_number,
            bind_id: self.bind_id,
            client_id: 0,
            connection_id: 0,
            cookies: std::ptr::null_mut(),
        }
    }

    pub fn get_count(&self) -> usize {
        let mut raw = self.as_raw();
        unsafe { webui_get_count(&mut raw) }
    }

    pub fn get_string(&self) -> String {
        let mut raw = self.as_raw();
        unsafe { char_to_string(webui_get_string(&mut raw)) }
    }

    pub fn get_string_at(&self, index: usize) -> String {
        let mut raw = self.as_raw();
        unsafe { char_to_string(webui_get_string_at(&mut raw, index)) }
    }

    pub fn get_int(&self) -> i64 {
        let mut raw = self.as_raw();
        unsafe { webui_get_int(&mut raw) }
    }

    pub fn get_int_at(&self, index: usize) -> i64 {
        let mut raw = self.as_raw();
        unsafe { webui_get_int_at(&mut raw, index) }
    }

    pub fn get_float(&self) -> f64 {
        let mut raw = self.as_raw();
        unsafe { webui_get_float(&mut raw) }
    }

    pub fn get_float_at(&self, index: usize) -> f64 {
        let mut raw = self.as_raw();
        unsafe { webui_get_float_at(&mut raw, index) }
    }

    pub fn get_bool(&self) -> bool {
        let mut raw = self.as_raw();
        unsafe { webui_get_bool(&mut raw) }
    }

    pub fn get_bool_at(&self, index: usize) -> bool {
        let mut raw = self.as_raw();
        unsafe { webui_get_bool_at(&mut raw, index) }
    }

    pub fn get_size(&self) -> usize {
        let mut raw = self.as_raw();
        unsafe { webui_get_size(&mut raw) }
    }

    pub fn get_size_at(&self, index: usize) -> usize {
        let mut raw = self.as_raw();
        unsafe { webui_get_size_at(&mut raw, index) }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let mut raw = self.as_raw();
        unsafe {
            let ptr = webui_get_string(&mut raw) as *const u8;
            let len = webui_get_size(&mut raw);
            std::slice::from_raw_parts(ptr, len).to_vec()
        }
    }

    pub fn get_bytes_at(&self, index: usize) -> Vec<u8> {
        let mut raw = self.as_raw();
        unsafe {
            let ptr = webui_get_string_at(&mut raw, index) as *const u8;
            let len = webui_get_size_at(&mut raw, index);
            std::slice::from_raw_parts(ptr, len).to_vec()
        }
    }

    pub fn return_int(&self, value: i64) {
        let mut raw = self.as_raw();
        unsafe { webui_return_int(&mut raw, value) }
    }

    pub fn return_float(&self, value: f64) {
        let mut raw = self.as_raw();
        unsafe { webui_return_float(&mut raw, value) }
    }

    pub fn return_string(&self, value: &str) {
        let mut raw = self.as_raw();
        let c_str = CString::new(value).unwrap();
        unsafe { webui_return_string(&mut raw, c_str.as_ptr()) }
    }

    pub fn return_bool(&self, value: bool) {
        let mut raw = self.as_raw();
        unsafe { webui_return_bool(&mut raw, value) }
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

    pub fn run_js(&self, script: impl AsRef<str>, buffer_size: usize) -> JavaScript {
        let mut js = JavaScript {
            timeout: 0,
            script: script.as_ref().to_string(),
            error: false,
            data: "".to_string(),
        };

        run_js_buffered(self.id, &mut js, if buffer_size == 0 { 1024 * 8 } else { buffer_size });

        js
    }

    pub fn set_icon(&self, icon: impl AsRef<str>, kind: impl AsRef<str>) {
        set_icon(self.id, icon.as_ref(), kind.as_ref());
    }

    pub fn set_file_handler(
        &self,
        handler: unsafe extern "C" fn(*const i8, *mut i32) -> *const std::os::raw::c_void,
    ) {
        set_file_handler(self.id, handler);
    }

    pub fn set_file_handler_window(
        &self,
        handler: unsafe extern "C" fn(usize, *const i8, *mut i32) -> *const std::os::raw::c_void,
    ) {
        set_file_handler_window(self.id, handler);
    }

    pub fn set_runtime(&self, runtime: WebUIRuntime) {
        set_runtime(self.id, runtime);
    }

    pub fn get_best_browser(&self) -> usize {
        get_best_browser(self.id)
    }

    pub fn show_wv(&self, content: impl AsRef<str>) -> bool {
        show_wv(self.id, content.as_ref())
    }

    pub fn start_server(&self, content: impl AsRef<str>) -> String {
        start_server(self.id, content.as_ref())
    }

    pub fn set_kiosk(&self, status: bool) {
        set_kiosk(self.id, status);
    }

    pub fn focus(&self) {
        focus(self.id);
    }

    pub fn set_custom_parameters(&self, params: impl AsRef<str>) {
        set_custom_parameters(self.id, params.as_ref());
    }

    pub fn set_high_contrast(&self, status: bool) {
        set_high_contrast(self.id, status);
    }

    pub fn set_resizable(&self, status: bool) {
        set_resizable(self.id, status);
    }

    pub fn minimize(&self) {
        minimize(self.id);
    }

    pub fn maximize(&self) {
        maximize(self.id);
    }

    pub fn set_root_folder(&self, path: impl AsRef<str>) -> bool {
        set_root_folder(self.id, path.as_ref())
    }

    pub fn set_close_handler_wv(
        &self,
        handler: unsafe extern "C" fn(usize) -> bool,
    ) {
        set_close_handler_wv(self.id, handler);
    }

    pub fn set_minimum_size(&self, width: u32, height: u32) {
        set_minimum_size(self.id, width, height);
    }

    pub fn set_center(&self) {
        set_center(self.id);
    }

    pub fn set_proxy(&self, proxy_server: impl AsRef<str>) {
        set_proxy(self.id, proxy_server.as_ref());
    }

    pub fn get_url(&self) -> String {
        get_url(self.id)
    }

    pub fn set_public(&self, status: bool) {
        set_public(self.id, status);
    }

    pub fn navigate(&self, url: impl AsRef<str>) {
        navigate(self.id, url.as_ref());
    }

    pub fn get_parent_process_id(&self) -> usize {
        get_parent_process_id(self.id)
    }

    pub fn get_child_process_id(&self) -> usize {
        get_child_process_id(self.id)
    }

    pub fn get_hwnd(&self) -> *mut std::os::raw::c_void {
        get_hwnd(self.id)
    }

    pub fn get_port(&self) -> usize {
        get_port(self.id)
    }

    pub fn set_port(&self, port: usize) -> bool {
        set_port(self.id, port)
    }

    pub fn set_event_blocking(&self, status: bool) {
        set_event_blocking(self.id, status);
    }

    pub fn set_frameless(&self, status: bool) {
        set_frameless(self.id, status);
    }

    pub fn set_transparent(&self, status: bool) {
        set_transparent(self.id, status);
    }

    pub fn run(&self, script: impl AsRef<str>) {
        run(self.id, script.as_ref());
    }

    pub fn send_raw(&self, func: impl AsRef<str>, raw: &[u8]) {
        send_raw(self.id, func.as_ref(), raw);
    }

    pub fn set_hide(&self, status: bool) {
        set_hide(self.id, status);
    }

    pub fn set_size(&self, width: u32, height: u32) {
        set_size(self.id, width, height);
    }

    pub fn set_position(&self, x: u32, y: u32) {
        set_position(self.id, x, y);
    }

    pub fn set_profile(&self, name: impl AsRef<str>, path: impl AsRef<str>) {
        set_profile(self.id, name.as_ref(), path.as_ref());
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
        // Note neccessary as WebUI already destroys the
        // window when it is closed.
        // destroy(self.id);
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

pub fn run_js_buffered(win: usize, js: &mut JavaScript, buffer_size: usize) {
    unsafe {
        let script_c_str = CString::new(js.script.clone()).unwrap();
        let mut buffer = vec![0i8; buffer_size];

        let ok = webui_script(
            win,
            script_c_str.as_ptr(),
            js.timeout,
            buffer.as_mut_ptr(),
            buffer_size,
        );

        js.error = !ok;
        js.data = char_to_string(buffer.as_ptr());
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

pub fn set_timeout(seconds: usize) {
    unsafe {
        webui_set_timeout(seconds);
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

pub fn set_icon(win: usize, icon: &str, kind: &str) {
    let icon_c_str = CString::new(icon).unwrap();
    let kind_c_str = CString::new(kind).unwrap();
    let icon_c_char: *const c_char = icon_c_str.as_ptr() as *const c_char;
    let kind_c_char: *const c_char = kind_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_set_icon(win, icon_c_char, kind_c_char);
    }
}

pub fn set_runtime(win: usize, runtime: WebUIRuntime) {
    unsafe {
        webui_set_runtime(win, runtime as usize);
    }
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

pub fn get_best_browser(win: usize) -> usize {
    unsafe { webui_get_best_browser(win) }
}

pub fn set_kiosk(win: usize, status: bool) {
    unsafe { webui_set_kiosk(win, status) }
}

pub fn show_wv(win: usize, content: impl AsRef<str> + Into<Vec<u8>>) -> bool {
    unsafe {
        let content_c_str = CString::new(content).unwrap();
        let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;
        webui_show_wv(win, content_c_char)
    }
}

pub fn start_server(win: usize, content: impl AsRef<str> + Into<Vec<u8>>) -> String {
    unsafe {
        let content_c_str = CString::new(content).unwrap();
        let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;
        let url = webui_start_server(win, content_c_char);
        char_to_string(url as *const i8)
    }
}

pub fn focus(win: usize) {
    unsafe { webui_focus(win) }
}

pub fn set_custom_parameters(win: usize, params: impl AsRef<str> + Into<Vec<u8>>) {
    let params_c_str = CString::new(params).unwrap();
    let params_c_char = params_c_str.as_ptr() as *mut c_char;
    unsafe { webui_set_custom_parameters(win, params_c_char) }
}

pub fn set_high_contrast(win: usize, status: bool) {
    unsafe { webui_set_high_contrast(win, status) }
}

pub fn set_resizable(win: usize, status: bool) {
    unsafe { webui_set_resizable(win, status) }
}

pub fn is_high_contrast() -> bool {
    unsafe { webui_is_high_contrast() }
}

pub fn browser_exist(browser: WebUIBrowser) -> bool {
    unsafe { webui_browser_exist(browser as usize) }
}

pub fn wait_async() -> bool {
    unsafe { webui_wait_async() }
}

pub fn minimize(win: usize) {
    unsafe { webui_minimize(win) }
}

pub fn maximize(win: usize) {
    unsafe { webui_maximize(win) }
}

pub fn set_browser_folder(path: impl AsRef<str> + Into<Vec<u8>>) {
    let path_c_str = CString::new(path).unwrap();
    let path_c_char: *const c_char = path_c_str.as_ptr() as *const c_char;
    unsafe { webui_set_browser_folder(path_c_char) }
}

pub fn set_close_handler_wv(win: usize, handler: unsafe extern "C" fn(usize) -> bool) {
    unsafe { webui_set_close_handler_wv(win, Some(handler)) }
}

pub fn set_file_handler(
    win: usize,
    handler: unsafe extern "C" fn(*const i8, *mut i32) -> *const std::os::raw::c_void,
) {
    unsafe {
        webui_set_file_handler(win, Some(handler));
    }
}

pub fn set_file_handler_window(
    win: usize,
    handler: unsafe extern "C" fn(usize, *const i8, *mut i32) -> *const std::os::raw::c_void,
) {
    unsafe {
        webui_set_file_handler_window(win, Some(handler));
    }
}

pub fn set_minimum_size(win: usize, width: u32, height: u32) {
    unsafe { webui_set_minimum_size(win, width, height) }
}

pub fn set_center(win: usize) {
    unsafe { webui_set_center(win) }
}

pub fn open_url(url: impl AsRef<str> + Into<Vec<u8>>) {
    let url_c_str = CString::new(url).unwrap();
    let url_c_char: *const c_char = url_c_str.as_ptr() as *const c_char;
    unsafe { webui_open_url(url_c_char) }
}

pub fn get_url(win: usize) -> String {
    unsafe {
        let url = webui_get_url(win);
        char_to_string(url as *const i8)
    }
}

pub fn set_public(win: usize, status: bool) {
    unsafe { webui_set_public(win, status) }
}

pub fn navigate(win: usize, url: impl AsRef<str> + Into<Vec<u8>>) {
    let url_c_str = CString::new(url).unwrap();
    let url_c_char: *const c_char = url_c_str.as_ptr() as *const c_char;
    unsafe { webui_navigate(win, url_c_char) }
}

pub fn get_parent_process_id(win: usize) -> usize {
    unsafe { webui_get_parent_process_id(win) }
}

pub fn get_child_process_id(win: usize) -> usize {
    unsafe { webui_get_child_process_id(win) }
}

pub fn get_hwnd(win: usize) -> *mut std::os::raw::c_void {
    unsafe { webui_get_hwnd(win) }
}

pub fn get_port(win: usize) -> usize {
    unsafe { webui_get_port(win) }
}

pub fn set_port(win: usize, port: usize) -> bool {
    unsafe { webui_set_port(win, port) }
}

pub fn get_free_port() -> usize {
    unsafe { webui_get_free_port() }
}

pub fn set_config(option: WebUIConfig, status: bool) {
    unsafe { webui_set_config(option.to_usize(), status) }
}

pub fn set_event_blocking(win: usize, status: bool) {
    unsafe { webui_set_event_blocking(win, status) }
}

pub fn set_frameless(win: usize, status: bool) {
    unsafe { webui_set_frameless(win, status) }
}

pub fn set_transparent(win: usize, status: bool) {
    unsafe { webui_set_transparent(win, status) }
}

pub fn get_mime_type(file: impl AsRef<str> + Into<Vec<u8>>) -> String {
    let file_c_str = CString::new(file).unwrap();
    let file_c_char: *const c_char = file_c_str.as_ptr() as *const c_char;
    unsafe { char_to_string(webui_get_mime_type(file_c_char) as *const i8) }
}

pub fn memcpy(dest: *mut std::os::raw::c_void, src: *mut std::os::raw::c_void, count: usize) {
    unsafe { webui_memcpy(dest, src, count) }
}

pub fn send_raw(win: usize, func: impl AsRef<str> + Into<Vec<u8>>, raw: &[u8]) {
    let func_c_str = CString::new(func).unwrap();
    let func_c_char: *const c_char = func_c_str.as_ptr() as *const c_char;
    unsafe {
        webui_send_raw(
            win,
            func_c_char,
            raw.as_ptr() as *const std::os::raw::c_void,
            raw.len(),
        )
    }
}

pub fn set_hide(win: usize, status: bool) {
    unsafe { webui_set_hide(win, status) }
}

pub fn set_size(win: usize, width: u32, height: u32) {
    unsafe { webui_set_size(win, width, height) }
}

pub fn set_position(win: usize, x: u32, y: u32) {
    unsafe { webui_set_position(win, x, y) }
}

pub fn set_profile(win: usize, name: &str, path: &str) {
    let name_c_str = CString::new(name).unwrap();
    let path_c_str = CString::new(path).unwrap();
    let name_c_char: *const c_char = name_c_str.as_ptr() as *const c_char;
    let path_c_char: *const c_char = path_c_str.as_ptr() as *const c_char;
    unsafe { webui_set_profile(win, name_c_char, path_c_char) }
}

pub fn set_proxy(win: usize, proxy_server: impl AsRef<str> + Into<Vec<u8>>) {
    let proxy_c_str = CString::new(proxy_server).unwrap();
    let proxy_c_char: *const c_char = proxy_c_str.as_ptr() as *const c_char;
    unsafe { webui_set_proxy(win, proxy_c_char) }
}

pub fn run(win: usize, script: impl AsRef<str> + Into<Vec<u8>>) {
    let script_c_str = CString::new(script).unwrap();
    let script_c_char: *const c_char = script_c_str.as_ptr() as *const c_char;
    unsafe { webui_run(win, script_c_char) }
}

pub fn get_last_error_number() -> usize {
    unsafe { webui_get_last_error_number() }
}

pub fn get_last_error_message() -> String {
    unsafe { char_to_string(webui_get_last_error_message() as *const i8) }
}

pub fn set_default_root_folder(path: impl AsRef<str> + Into<Vec<u8>>) -> bool {
    let path_c_str = CString::new(path).unwrap();
    let path_c_char: *const c_char = path_c_str.as_ptr() as *const c_char;
    unsafe { webui_set_default_root_folder(path_c_char) }
}

pub fn set_root_folder(win: usize, path: impl AsRef<str> + Into<Vec<u8>>) -> bool {
    let path_c_str = CString::new(path).unwrap();
    let path_c_char: *const c_char = path_c_str.as_ptr() as *const c_char;
    unsafe { webui_set_root_folder(win, path_c_char) }
}

pub fn set_tls_certificate(certificate_pem: &str, private_key_pem: &str) -> bool {
    let cert_c_str = CString::new(certificate_pem).unwrap();
    let key_c_str = CString::new(private_key_pem).unwrap();
    let cert_c_char: *const c_char = cert_c_str.as_ptr() as *const c_char;
    let key_c_char: *const c_char = key_c_str.as_ptr() as *const c_char;
    unsafe { webui_set_tls_certificate(cert_c_char, key_c_char) }
}

pub fn clean() {
    unsafe { webui_clean() }
}

pub fn delete_all_profiles() {
    unsafe { webui_delete_all_profiles() }
}

pub fn delete_profile(win: usize) {
    unsafe { webui_delete_profile(win) }
}
