#![feature(libc)]

extern crate libc;
#[macro_use]
extern crate redhook;

use libc::{size_t, ssize_t, c_char, c_int, c_void};
use std::ffi::CString;

// hook! {
//     unsafe fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t => my_readlink {
//         if let Ok(path) = std::str::from_utf8(std::ffi::CStr::from_ptr(path).to_bytes()) {
//             println!("readlink(\"{}\")", path);
//         } else {
//             println!("readlink(...)");
//         }
//
//         real!(readlink)(path, buf, bufsiz)
//     }
// }


hook! {
    unsafe fn puts(s: *const c_char) -> c_int => custom_puts {
        match std::str::from_utf8(std::ffi::CStr::from_ptr(s).to_bytes()) {
            Ok(s) => println!("intercepting puts to print '{}'", s),
            Err(e) => println!("error decoding bytes: {}", e),
        }
        real!(puts)(s)
    }
}


// FIXME the real signature for printf has varargs, but not sure how to express this in rust
hook! {
    unsafe fn printf(format: *const c_char) -> c_int => custom_printf {
        match std::str::from_utf8(std::ffi::CStr::from_ptr(format).to_bytes()) {
            Ok(s) => println!("intercepting printf call to print '{}'", s),
            Err(e) => println!("error decoding bytes: {}", e),
        }
        real!(printf)(format)
    }
}

hook! {
    unsafe fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t => custom_write {
        if fd == 1 || fd == 2 { // i.e. stdout or stdin
            do_println(format!("write({fd}, {buf}, {count})", fd=fd, buf="<buf is not renderable yet>", count=count));
        }

        real!(write)(fd, buf, count)
    }
}

// Since we're hooking functions that println() uses, attempting to use println! directly causes a crash.
// So instead use: do_println(format!("something {}", my_var));
fn do_println<S: std::fmt::Display>(s: S) {
    let c_to_print = CString::new(format!("{}", s)).unwrap();
    unsafe {
        real!(puts)(c_to_print.as_ptr());
    }
}
