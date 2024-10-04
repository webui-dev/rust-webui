use std::ffi::CString;
use std::os::raw::c_char;

use crate::bindgen::*;
use crate::char_to_string;

// Events
pub type EventType = webui_event;

// Implement into<usize>
impl EventType {
    pub fn from_usize(value: usize) -> EventType {
        match value {
            0 => EventType::WEBUI_EVENT_DISCONNECTED,
            1 => EventType::WEBUI_EVENT_CONNECTED,
            2 => EventType::WEBUI_EVENT_MOUSE_CLICK,
            3 => EventType::WEBUI_EVENT_NAVIGATION,
            4 => EventType::WEBUI_EVENT_CALLBACK,
            _ => EventType::WEBUI_EVENT_CALLBACK,
        }
    }
}

pub struct EventSimple {
    pub win: usize,
    pub event_type: EventType,
    pub element: String,
    pub event_number: usize,
    pub bind_id: usize,
}

impl EventSimple {
    pub fn set_response(&self, response: &str) {
        // interface_set_response(self.window, self.event_number, response);
        let response_c_str = CString::new(response).unwrap();
        let response_c_char: *const c_char = response_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_interface_set_response(self.win, self.event_number, response_c_char);
        }
    }

    pub fn get_string_at(&self, index: usize) -> String {
        unsafe {
            let string = webui_interface_get_string_at(self.win, self.event_number, index);
            char_to_string(string)
        }
    }

    pub fn get_int_at(&self, index: usize) -> i64 {
        unsafe { webui_interface_get_int_at(self.win, self.event_number, index) }
    }

    pub fn get_float_at(&self, index: usize) -> f64 {
        unsafe { webui_interface_get_float_at(self.win, self.event_number, index) }
    }

    pub fn get_bool_at(&self, index: usize) -> bool {
        unsafe { webui_interface_get_bool_at(self.win, self.event_number, index) }
    }

    pub fn get_size_at(&self, index: usize) -> usize {
        unsafe { webui_interface_get_size_at(self.win, self.event_number, index) }
    }
}

pub struct Event {
    pub win: usize,
    pub event_type: EventType,
    pub element: String,
    pub event_number: usize,
    pub bind_id: usize,
    pub client_id: usize,
    pub connection_id: usize,
    pub cookies: String,
    event: *mut webui_event_t,
}

impl Event {
    pub fn new(event: *mut webui_event_t) -> Event {
        unsafe {
            let win = (*event).window;
            let event_type = EventType::from_usize((*event).event_type);
            let element = char_to_string((*event).element);
            let event_number = (*event).event_number;
            let bind_id = (*event).bind_id;
            let client_id = (*event).client_id;
            let connection_id = (*event).connection_id;
            let cookies = char_to_string((*event).cookies);

            Event {
                win,
                event_type,
                element,
                event_number,
                bind_id,
                client_id,
                connection_id,
                cookies,
                event,
            }
        }
    }

    pub fn show_client(&self, content: impl AsRef<str> + Into<Vec<u8>>) -> bool {
        unsafe {
            // Content String to i8/u8
            let content_c_str = CString::new(content).unwrap();
            let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

            webui_show_client(self.event, content_c_char)
        }
    }

    pub fn close_client(self) {
        unsafe {
            webui_close_client(self.event);
        }
    }

    pub fn send_raw(&self, function: &str, data: &[u8]) {
        let size = data.len();
        let raw = data.as_ptr() as *mut std::os::raw::c_void;
        let function_c_str = CString::new(function).unwrap();
        let function_c_char: *const c_char = function_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_send_raw_client(self.event, function_c_char, raw, size);
        }
    }

    pub fn navigate_client(&self, url: &str) {
        let url_c_str = CString::new(url).unwrap();
        let url_c_char: *const c_char = url_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_navigate_client(self.event, url_c_char);
        }
    }

    pub fn run(&self, script: &str) {
        let script_c_str = CString::new(script).unwrap();
        let script_c_char: *const c_char = script_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_run_client(self.event, script_c_char);
        }
    }

    pub fn script(&self, script: &str, timeout: usize, buffer_length: usize) -> Result<String, ()> {
        let script_c_str = CString::new(script).unwrap();
        let script_c_char: *const c_char = script_c_str.as_ptr() as *const c_char;

        let buffer_c_str = CString::new(vec![0; buffer_length]).unwrap();
        let buffer_c_char: *mut c_char = buffer_c_str.as_ptr() as *mut c_char;

        unsafe {
            match webui_script_client(
                self.event,
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

    pub fn get_count(&self) -> usize {
        unsafe { webui_get_count(self.event) }
    }

    pub fn get_int_at(&self, index: usize) -> i64 {
        unsafe { webui_get_int_at(self.event, index) }
    }

    pub fn get_int(&self) -> i64 {
        unsafe { webui_get_int(self.event) }
    }

    pub fn get_float_at(&self, index: usize) -> f64 {
        unsafe { webui_get_float_at(self.event, index) }
    }

    pub fn get_float(&self) -> f64 {
        unsafe { webui_get_float(self.event) }
    }

    pub fn get_string_at(&self, index: usize) -> String {
        unsafe {
            let string = webui_get_string_at(self.event, index);
            char_to_string(string)
        }
    }

    pub fn get_string(&self) -> String {
        unsafe {
            let string = webui_get_string(self.event);
            char_to_string(string)
        }
    }

    pub fn get_bool_at(&self, index: usize) -> bool {
        unsafe { webui_get_bool_at(self.event, index) }
    }

    pub fn get_bool(&self) -> bool {
        unsafe { webui_get_bool(self.event) }
    }

    pub fn get_size_at(&self, index: usize) -> usize {
        unsafe { webui_get_size_at(self.event, index) }
    }

    pub fn get_size(&self) -> usize {
        unsafe { webui_get_size(self.event) }
    }

    pub fn return_int(&self, value: i64) {
        unsafe {
            webui_return_int(self.event, value);
        }
    }

    pub fn return_float(&self, value: f64) {
        unsafe {
            webui_return_float(self.event, value);
        }
    }

    pub fn return_string(&self, value: &str) {
        let value_c_str = CString::new(value).unwrap();
        let value_c_char: *const c_char = value_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_return_string(self.event, value_c_char);
        }
    }

    pub fn return_bool(&self, value: bool) {
        unsafe {
            webui_return_bool(self.event, value);
        }
    }
}
