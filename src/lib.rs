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

use std::time::Duration;
use std::ops::Drop;

use std::sync::Arc;

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

struct UnsafeClient {
    client_ptr: *mut c_void
}

impl UnsafeClient {
    fn new() -> Self {
        unsafe {
            UnsafeClient {
                client_ptr: td_json_client_create()
            }
        }
    }

    /// UNSAFE: the returned slice is invalidated upon the next call to execute or receive
    unsafe fn execute<'a>(&'a self, request: &str) -> Option<&'a str> {
        let crequest = CString::new(request).expect("null character in request string");
        let answer = td_json_client_execute(
            self.client_ptr,
            crequest.as_ptr() as *const c_char
        );

        let answer = answer as *const c_char;
        if answer == std::ptr::null() {
            return None;
        }
        let answer = CStr::from_ptr(answer);
        Some(answer.to_str().expect("tdlib sent invalid utf-8 string"))
    }

    fn send(&self, request: &str) {
        let crequest = CString::new(request).expect("null character in request string");
        unsafe {
            td_json_client_send(
                self.client_ptr,
                crequest.as_ptr() as *const c_char
            )
        }
    }

    /// UNSAFE: the returned slice is invalidated upon the next call to execute or receive
    unsafe fn receive<'a>(&'a self, timeout: Duration) -> Option<&'a str> {
        let timeout = timeout.as_secs() as f64;

        let answer = td_json_client_receive(
            self.client_ptr,
            timeout
        );

        let answer = answer as *const c_char;
        if answer == std::ptr::null() {
            return None;
        }
        let answer = CStr::from_ptr(answer);

        Some(answer.to_str().expect("tdlib sent invalid utf-8 string"))
    }
}

impl Drop for UnsafeClient {
    fn drop(&mut self) {
        unsafe {
            td_json_client_destroy(self.client_ptr)
        }
    }
}

pub struct Client {
    inner: UnsafeClient,
}

impl Client {
    pub fn new() -> Self {
        Client {
            inner: UnsafeClient::new(),
        }
    }

    pub fn execute<'a>(&'a mut self, request: &str) -> Option<&'a str> {
        // SAFE because we are taking self by mutable referene
        unsafe {
            self.inner.execute(request)
        }
    }

    pub fn send(&self, request: &str) {
        self.inner.send(request)
    }

    pub fn receive<'a>(&'a mut self, timeout: Duration) -> Option<&'a str> {
        // SAFE because we are taking self by mutable referene
        unsafe {
            self.inner.receive(timeout)
        }
    }
    pub fn split(self) -> (SendClient, ReceiveClient) {
        let c = Arc::new(self.inner);
        let s = SendClient {
            inner: c.clone(),
        };
        let r = ReceiveClient {
            inner: c.clone(),
        };
        (s, r)
    }
}

#[derive(Clone)]
pub struct SendClient {
    inner: Arc<UnsafeClient>,
}
pub struct ReceiveClient {
    inner: Arc<UnsafeClient>,
}

/// SAFE because the send method can be called by any threads
unsafe impl Send for SendClient{}
/// SAFE because the send method can be called by multiple threads at the same time
unsafe impl Sync for SendClient{}

impl SendClient {
    pub fn execute<'a>(&'a mut self, request: &str) -> Option<&'a str> {
        // SAFE because we are taking self by mutable referene
        unsafe {
            self.inner.execute(request)
        }
    }

    pub fn send(&self, request: &str) {
        self.inner.send(request);
    }
}

/// SAFE because the send method can be called by any threads
unsafe impl Send for ReceiveClient{}
impl ReceiveClient {
    pub fn receive<'a>(&'a mut self, timeout: Duration) -> Option<&'a str> {
        // SAFE because we are taking self by mutable referene
        unsafe {
            self.inner.receive(timeout)
        }
    }
}
