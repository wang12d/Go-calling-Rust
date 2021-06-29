extern crate libc;

use libc::c_char;
use std::ffi::CString;
use std::sync::{Once, ONCE_INIT};
use std::mem::transmute;
use std::ptr::read;

static START: Once = ONCE_INIT;
static mut data:*const CString = 0 as *const CString;

#[no_mangle]
pub extern fn hello() -> *const c_char {
    START.call_once(|| {
        unsafe {
            let boxed = Box::new(CString::new("Hello World").expect("CString::new failed"));
            data = transmute(boxed);
        }
    });
    unsafe {
        return (&*data).as_ptr();
    }
}
