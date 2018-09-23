extern crate tdjson_sys;

use tdjson_sys::*;

use std::os::raw::{
    c_void,
    c_char,
};

use std::ffi::{
    CString,
    CStr,
};

use std::str::Utf8Error;
use std::time::Duration;
use std::ops::Drop;

pub fn set_log_file(path: &str) -> Result<i32, std::ffi::NulError> {
    let cpath = CString::new(path)?;
    unsafe {
        Ok(td_set_log_file_path(cpath.as_ptr()))
    }
}

pub fn set_log_verbosity_level(level : i32) {
    unsafe {
        td_set_log_verbosity_level(level);
    }
}

pub struct Client {
    client_ptr: *mut c_void
}

impl Client {
    pub fn new() -> Self {
        unsafe {
            Client {
                client_ptr: td_json_client_create()
            }
        }
    }

    pub fn execute<'a>(&'a mut self, request: &str) -> Result<Option<&'a str>, Utf8Error> {
        let crequest = CString::new(request).expect("null character in request string");
        unsafe {
            let answer = td_json_client_execute(
                self.client_ptr,
                crequest.as_ptr() as *const c_char
            );

            let answer = answer as *const c_char;
            if answer == std::ptr::null() {
                return Ok(None);
            }
            let answer = CStr::from_ptr(answer);
            answer.to_str().map(|s| Some(s))
        }
    }

    pub fn send(&mut self, request: &str) {
        let crequest = CString::new(request).expect("null character in request string");
        unsafe {
            td_json_client_send(
                self.client_ptr,
                crequest.as_ptr() as *const c_char
            )
        }
    }

    pub fn receive<'a>(&'a mut self, timeout: Duration) -> Result<Option<&'a str>, Utf8Error> {
        let timeout = timeout.as_secs() as f64;

        unsafe {
            let answer = td_json_client_receive(
                self.client_ptr,
                timeout
            );

            let answer = answer as *const c_char;
            if answer == std::ptr::null() {
                return Ok(None);
            }
            let answer = CStr::from_ptr(answer);

            answer.to_str().map(|s| Some(s))
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            td_json_client_destroy(self.client_ptr)
        }
    }
}
