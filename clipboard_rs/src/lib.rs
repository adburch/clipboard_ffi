mod bindings;

use std::ffi::OsString;
use std::os::windows::prelude::*;

use bindings::*;

#[no_mangle]
pub extern "C" fn print_clipboard_file(filename: *const u16, len: usize) {
    let wchar_slice = unsafe { std::slice::from_raw_parts(filename, len) };
    let filename_os = OsString::from_wide(wchar_slice);
    if let Ok(filename) = filename_os.into_string() {
        println!("filename is {}", filename);
    } else {
        println!("filename is incorrect")
    }

    unsafe {
    match OpenClipboard(0) {
        0 => println!("Clipboard failed to open: {}", GetLastError()),
        _ => println!("Clipboard is open")
    }

    CloseClipboard();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
