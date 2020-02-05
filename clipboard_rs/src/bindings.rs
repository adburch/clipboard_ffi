use std::ffi::c_void;

extern "system" {
    pub fn OpenClipboard(hWndNewOwner: u32) -> u32;
    pub fn CloseClipboard();
    pub fn GetLastError() -> u32;
}

pub struct ClipboardOpener{  }

impl ClipboardOpener {
    pub fn new(hWndNewOwner: Option<u32>) -> Result<ClipboardOpener,u32> {
        unsafe {
            match OpenClipboard(hWndNewOwner.unwrap_or(0)) {
                0 => {
                    let err = GetLastError();
                    println!("Clipboard failed to open: {}", err);
                    Err(err)
                },
                _ => {
                    println!("Clipboard is open");
                    Ok(ClipboardOpener{})
                }
            }
        }
    }
}

impl Drop for ClipboardOpener {
    fn drop(&mut self) {
        unsafe { CloseClipboard(); }
    }
}


extern "system" {

    pub fn IsClipboardFormatAvailable(format: u32) -> u32;

    pub fn GetClipboardData(format: u32) -> u32;

    pub fn GlobalLock(hMem: u32) -> *mut c_void;

    pub fn GlobalUnlock(hMem: u32);
}