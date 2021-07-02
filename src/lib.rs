#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

pub mod ffi;

#[test]
fn test() {
    unsafe {
        println!("{}", ffi::archive_version_number());
    }
}
