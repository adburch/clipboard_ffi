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
            report_error(
                "filename is incorrect",
                0xffffffff,
                std::line!(),
                std::file!(),
            );
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

    // let clipboard = match ClipboardGuard::new(None) {
    //     Ok(c) => c,
    //     _ => return,
    // };

    // let clpbd_txt = match clipboard.get_clipboard_text() {
    //     Ok(c) => c,
    //     _ => return,
    // };

    // let clpbd_str = string_from_lpcwstr(*clpbd_txt.lock_data().unwrap()).unwrap();

    // let mut file = File::create(filename).unwrap();
    // file.write_all(clpbd_str.as_bytes()).unwrap();
    // file.sync_all().unwrap();

    prnt_clpbd_file(&filename).unwrap();
}

fn prnt_clpbd_file(filename: &str) -> Result<(), ()> {
    // let clipboard = match ClipboardGuard::new(Nodne) {
    //     Ok(c) => c,
    //     _ => return,
    // };
    let clipboard = ClipboardGuard::new(None)?;

    // let clpbd_txt = match clipboard.get_clipboard_text() {
    //     Ok(c) => c,
    //     _ => return,
    // };
    let clpbd_txt = clipboard.get_clipboard_text()?;

    let clpbd_str = string_from_lpcwstr(*clpbd_txt.lock_data()?)?;

    write_text_to_file(filename, &clpbd_str)
}

fn write_text_to_file(filename: &str, text: &str) -> Result<(), ()> {
    let mut file = File::create(filename).map_err(|e| {
        report_error(
            "Failed to open file",
            e.raw_os_error().unwrap_or(-1) as u32,
            std::line!(),
            std::file!(),
        )
    })?;
    file.write_all(text.as_bytes()).map_err(|e| {
        report_error(
            "Failed to write to file",
            e.raw_os_error().unwrap_or(-1) as u32,
            std::line!(),
            std::file!(),
        )
    })?;
    file.sync_all().map_err(|e| {
        report_error(
            "Failed to sync file",
            e.raw_os_error().unwrap_or(-1) as u32,
            std::line!(),
            std::file!(),
        )
    })
}

pub fn string_from_wchar(ptr: *const u16, len: usize) -> Result<String, ()> {
    let wchar_slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let str_os = OsString::from_wide(wchar_slice);
    str_os.into_string().map_err(|_| ())
}
