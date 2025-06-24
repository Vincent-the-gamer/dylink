use std::ffi::{c_char, CStr};

use crate::notification::use_notification;

mod notification;

#[no_mangle]
pub extern "C" fn send_notification(title: *const c_char, body: *const c_char) -> () {
    unsafe {
        let title = CStr::from_ptr(title).to_str().expect("Parse title to &str error!");
        let body = CStr::from_ptr(body).to_str().expect("Parse body to &str error!");
        use_notification(title, body);
    }
}

