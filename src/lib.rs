#![feature(libc)]

extern crate libc;
#[macro_use]
extern crate redhook;

use libc::{size_t, ssize_t, c_char, c_int, c_void};
use std::ffi::{self, CString};
use std::str;

hook! {
    unsafe fn puts(line: *const c_char) -> c_int => custom_puts {
        // we assume that line is valid unicode
        let line_as_str = str::from_utf8_unchecked(ffi::CStr::from_ptr(line).to_bytes());
        let line_with_thread_ids = CString::new(format!("{}{}", add_thread_id_before_newlines(line_as_str), get_thread_id_as_string()))
            .unwrap();

        real!(puts)(line_with_thread_ids.as_ptr())
    }
}

// extern {
//     fn printf(format: *const c_char, ...) -> c_int;
//  }

// FIXME doesn't work properly (varargs are ignored) because the real signature for printf has varargs
// while varags are valid in extern decls, redhook doesn't support it in its macro syntax
hook! {
    unsafe fn printf(format: *const c_char) -> c_int => custom_printf {
        // we assume that format is valid unicode
        let format_as_str = str::from_utf8_unchecked(ffi::CStr::from_ptr(format).to_bytes());
        let format_with_thread_ids = CString::new(add_thread_id_before_newlines(format_as_str))
            .unwrap();

        real!(printf)(format_with_thread_ids.as_ptr())
    }
}

//TODO vprintf
//TODO vfprintf
//TODO fprintf

hook! {
    unsafe fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t => custom_write {
        if (fd == 1 || fd == 2) && count > 0 { // i.e. stdout or stdin
            // we assume that buf is valid unicode
            let buf_as_str = str::from_utf8_unchecked(std::slice::from_raw_parts(buf as *const u8, count));

            let buf_with_thread_ids = add_thread_id_before_newlines(buf_as_str);

            let slice_to_write = buf_with_thread_ids.as_bytes();
            let bytes_written = real!(write)(fd, slice_to_write.as_ptr() as *const c_void, slice_to_write.len());
            if bytes_written == slice_to_write.len() as isize {
                // huge success; lie by pretending only the requested bytes were written, even though we added more
                count as ssize_t
            } else if bytes_written < count as isize {
                // we don't know whether bytes_written includes our added data, so we escalate this to a full failure
                -1
            } else {
                // actual failure, just propagate it
                bytes_written
            }
        } else {
            real!(write)(fd, buf, count)
        }
    }
}

// Since we're hooking functions that println() uses, attempting to use println! directly causes a crash.
// So instead use: do_println(format!("something {}", my_var));
#[allow(dead_code)]
fn do_println<S: std::fmt::Display>(s: S) {
    let c_to_print = CString::new(format!("{}", s)).unwrap();
    unsafe {
        real!(puts)(c_to_print.as_ptr());
    }
}

fn add_thread_id_before_newlines(s: &str) -> String {
    let thread_id_with_newline = format!("{}\n", get_thread_id_as_string());
    s.replace("\n", thread_id_with_newline.as_str())
}

fn get_thread_id_as_string() -> String {
    format!("[ThreadId={}]", std::thread::current().id())
}

trait IdentifiableThread {
    fn id(&self) -> u64;
}

impl IdentifiableThread for std::thread::Thread {
    fn id(&self) -> u64 {
        unsafe { libc::pthread_self() }
    }
}
