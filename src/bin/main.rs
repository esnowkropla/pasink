#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate user32;

extern crate pasink;

use std::env;
use pasink::{server, client};

#[cfg(windows)]
fn open_window() {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::winuser::MB_OK;
    use user32::MessageBoxW;
    let wide: Vec<u16> = OsStr::new("hello").encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK);
    };
}

#[cfg(not(windows))]
fn open_window() {}

fn main() {
    /* open_window(); */
    let args: Vec<String> = env::args().collect();
    if args[1] == "server" {
        server(args[2].as_str());
    } else if args[1] == "client" {
        client(args[2].as_str());
    }
}
