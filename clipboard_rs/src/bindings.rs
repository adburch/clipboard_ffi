use std::ffi::c_void;

extern "C" {
    pub fn OpenClipboard(hWndNewOwner: u32) -> u32;

    pub fn IsClipboardFormatAvailable(format: u32) -> u32;

    pub fn GetClipboardData(format: u32) -> u32;

    pub fn GlobalLock(hMem: u32) -> *mut c_void;

    pub fn GlobalUnlock(hMem: u32);

    pub fn CloseClipboard();
}