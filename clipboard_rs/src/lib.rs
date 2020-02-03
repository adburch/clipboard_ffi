mod bindings;

use std::ffi::OsString;
use std::os::windows::prelude::*;

#[no_mangle]
pub extern "C" fn print_clipboard_file(filename: *const u16, len: usize) {
    let wchar_slice =
    unsafe {
        std::slice::from_raw_parts(filename, len)
    };
    let filename_os = OsString::from_wide(wchar_slice);
    if let Ok(filename) = filename_os.into_string() {
        println!("filename is {}", filename);
    } else {
        println!("borkde")
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
