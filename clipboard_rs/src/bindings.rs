use std::ffi::c_void;

extern "system" {
    pub fn OpenClipboard(hWndNewOwner: u32) -> u32;

    pub fn GetLastError() -> u32;

    pub fn IsClipboardFormatAvailable(format: u32) -> u32;

    pub fn GetClipboardData(format: u32) -> u32;

    pub fn GlobalLock(hMem: u32) -> *mut c_void;

    pub fn GlobalUnlock(hMem: u32);

    pub fn CloseClipboard();
}

// pub struct ClipboardOpener{}

// impl ClipboardOpener {
//     pub fn new(Option<u32>)
// }