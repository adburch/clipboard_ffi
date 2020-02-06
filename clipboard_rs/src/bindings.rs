use std::ops::Deref;
use std::ffi::OsString;
use std::os::windows::prelude::*;

use winapi::shared::ntdef::NULL;
use winapi::um::winbase::{lstrlenW, GlobalLock, GlobalUnlock};
use winapi::um::winnt::{HANDLE, LPCWSTR};
use winapi::um::winuser::{GetClipboardData, CF_UNICODETEXT};

include!(concat!(env!("OUT_DIR"), "/generated_bindings.rs"));

use crate::string_from_wchar;

extern "system" {
    pub fn OpenClipboard(hWndNewOwner: u32) -> u32;
    pub fn CloseClipboard();
    pub fn GetLastError() -> u32;
}

fn get_last_error() -> u32 {
    unsafe { GetLastError() }
}

pub struct ClipboardGuard {}

impl ClipboardGuard {
    pub fn new(h_wnd_new_owner: Option<u32>) -> Result<ClipboardGuard, ()> {
        unsafe {
            match OpenClipboard(h_wnd_new_owner.unwrap_or(0)) {
                0 => {
                    let err = GetLastError();
                    report_error("Clipboard failed to open", err, std::line!(), std::file!());
                    Err(())
                }
                _ => {
                    // println!("Clipboard is open");
                    Ok(ClipboardGuard {})
                }
            }
        }
    }

    pub fn get_clipboard_text<'a>(&'a self) -> Result<ClipboardTextData<'a>, ()> {
        let data = unsafe { GetClipboardData(CF_UNICODETEXT) };
        match data {
            NULL => {
                let err = get_last_error();
                report_error("No text on clipboard", err, std::line!(), std::file!());
                Err(())
            }
            h => Ok(ClipboardTextData {
                data_handle: h,
                _clipboard: self,
            }),
        }
    }
}

impl Drop for ClipboardGuard {
    fn drop(&mut self) {
        unsafe {
            CloseClipboard();
        }
    }
}

pub struct ClipboardTextData<'a> {
    _clipboard: &'a ClipboardGuard,
    data_handle: HANDLE,
}

impl<'a> ClipboardTextData<'a> {
    pub fn lock_data(&'a self) -> Result<GlobalLockGuard<'a>, ()> {
        let text_ptr = unsafe { GlobalLock(self.data_handle) };
        match text_ptr {
            NULL => {
                let err = get_last_error();
                report_error("Unable to lock clipboard data", err, std::line!(), std::file!());
                Err(())
            }
            h => Ok(GlobalLockGuard {
                clpbd_data: self,
                text: h as LPCWSTR,
            }),
        }
    }
}

pub struct GlobalLockGuard<'a> {
    clpbd_data: &'a ClipboardTextData<'a>,
    text: LPCWSTR,
}

impl<'a> Drop for GlobalLockGuard<'a> {
    fn drop(&mut self) {
        unsafe {
            GlobalUnlock(self.clpbd_data.data_handle);
        }
    }
}

impl<'a> Deref for GlobalLockGuard<'a> {
    type Target = LPCWSTR;

    fn deref(&self) -> &LPCWSTR {
        &self.text
    }
}

pub fn string_from_lpcwstr(text_ptr: LPCWSTR) -> Result<String, ()> {
    let str_len = unsafe { lstrlenW(text_ptr) };
    string_from_wchar(text_ptr, str_len as usize)
}

pub fn report_error(msg: &str, error_code: u32, line: u32, file: &str) {
    let mut msg_os = OsString::new();
    msg_os.push(msg);
    let mut file_os = OsString::new();
    file_os.push(file);
    unsafe { ReportError ( ErrorRecord { Message: msg.as_ptr() as *const u16, ErrorCode: error_code, LineNumber: line, File: file.as_ptr() as *const u16 });}
}