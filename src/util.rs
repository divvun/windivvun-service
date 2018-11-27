use std::io;
use std::ffi::{OsStr, OsString};
use std::os::windows::prelude::*;
use winapi::shared::guiddef::GUID;
use winapi::um::winnls::ResolveLocaleName;
use winapi::ctypes::c_int;

const LOCALE_NAME_MAX_LENGTH: usize = 85;

pub fn to_u16s<S: AsRef<OsStr>>(s: S) -> io::Result<Vec<u16>> {
    fn inner(s: &OsStr) -> io::Result<Vec<u16>> {
        let mut maybe_result: Vec<u16> = s.encode_wide().collect();
        if maybe_result.iter().any(|&u| u == 0) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput,
                                      "strings passed to WinAPI cannot contain NULs"));
        }
        maybe_result.push(0);
        Ok(maybe_result)
    }
    inner(s.as_ref())
}

pub unsafe fn u16_ptr_to_string(ptr: *const u16) -> OsString {
    let len = (0..).take_while(|&i| *ptr.offset(i) != 0).count();
    let slice = std::slice::from_raw_parts(ptr, len);

    OsString::from_wide(slice)
}

pub fn fmtGuid(guid: &GUID) -> String {
    format!("{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}", guid.Data1, guid.Data2, guid.Data3, guid.Data4[0], guid.Data4[1], guid.Data4[2], guid.Data4[3], guid.Data4[4], guid.Data4[5], guid.Data4[6], guid.Data4[7])
}

pub fn resolve_locale_name(tag: &str) -> Option<String> {
    let mut buf = vec![0u16; LOCALE_NAME_MAX_LENGTH];

    let tag_wide;
    match to_u16s(tag) {
        Err(_) => return None,
        Ok(tag) => tag_wide = tag
    };
    
    let ret = unsafe {
        ResolveLocaleName(
            tag_wide.as_ptr(),
            buf.as_mut_ptr(),
            buf.len() as c_int
        )
    };
    
    if ret == 0 {
        let err = io::Error::last_os_error();
        error!("{:?}", err);
        panic!();
    }

    buf.truncate(ret as usize - 1);

    if buf.len() == 0 {
        return None;
    }

    Some(OsString::from_wide(&buf).into_string().unwrap())
}