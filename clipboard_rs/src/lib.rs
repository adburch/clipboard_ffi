mod bindings;

use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::os::windows::prelude::*;

use bindings::*;

#[no_mangle]
pub extern "C" fn print_clipboard_file(filename: *const u16, len: usize) {
    // let wchar_slice = unsafe { std::slice::from_raw_parts(filename, len) };
    // let filename_os = OsString::from_wide(wchar_slice);
    // if let Ok(filename) = filename_os.into_string() {
    let filename = match string_from_wchar(filename, len) {
        Ok(f) => {
            println!("filename is {}", f);
            f
        }
        Err(_) => {
            println!("filename is incorrect");
            return;
        }
    };

    // unsafe {
    // match OpenClipboard(0) {
    //     0 => println!("Clipboard failed to open: {}", GetLastError()),
    //     _ => println!("Clipboard is open")
    // }

    // CloseClipboard();
    // }

    let clipboard = match ClipboardGuard::new(None) {
        Ok(c) => c,
        _ => return,
    };

    let clpbd_txt = match clipboard.get_clipboard_text() {
        Ok(c) => c,
        _ => return,
    };

    let clpbd_str = string_from_lpcwstr(*clpbd_txt.lock_data().unwrap()).unwrap();

    let mut file = File::create(filename).unwrap();
    file.write_all(clpbd_str.as_bytes()).unwrap();
    file.sync_all().unwrap();
}

pub fn string_from_wchar(ptr: *const u16, len: usize) -> Result<String, ()> {
    let wchar_slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let str_os = OsString::from_wide(wchar_slice);
    str_os.into_string().map_err(|_| ())
}
