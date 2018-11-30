use std::io;
use std::ffi::{OsStr, OsString};
use std::os::windows::prelude::*;
use winapi::shared::guiddef::GUID;
use winapi::um::winnls::ResolveLocaleName;
use winapi::ctypes::c_int;
use winapi::um::combaseapi::CoTaskMemAlloc;
use winapi::um::winbase;
use std::mem;
use winapi::shared::wtypesbase::{LPOLESTR, OLECHAR};

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
    let len = winbase::lstrlenW(ptr) as usize;
    let slice = std::slice::from_raw_parts(ptr, len);

    OsString::from_wide(slice)
}

pub fn wide_from_slice(slice: &[u16]) -> OsString {
    let len = (0..).take_while(|&i| slice[i] != 0).count();
    OsString::from_wide(&slice[0..len])
}

pub unsafe fn alloc_com_str<S: AsRef<OsStr>>(s: S) -> Option<*mut OLECHAR> {
    unsafe fn inner(s: &OsStr) -> Option<*mut OLECHAR> {
        let s_vec = to_u16s(s).unwrap();
        let str_size = s_vec.len() * mem::size_of::<OLECHAR>();
        let elem_str = CoTaskMemAlloc(str_size) as *mut OLECHAR;

        info!("Str {:?} size {}, ptr {:?}", s, str_size, elem_str);
        // Copy string
        let str_slice: &[u16] = &s_vec;
        std::ptr::copy_nonoverlapping(str_slice.as_ptr(), elem_str, s_vec.len());
        Some(elem_str)
    }
    inner(s.as_ref())
}

/// Returns the path to the currently loaded module on Windows (DLL or executable), by getting the module handle
/// from a function in the module (get_module_path itself) and returning its filename.
pub fn get_module_path() -> Option<OsString> {
    use winapi::um::libloaderapi as ll;
    use winapi::shared::minwindef::{HMODULE, MAX_PATH};

    let mut handle: HMODULE = std::ptr::null_mut();
    let func = get_module_path as *const u16;
    let res = unsafe {
        ll::GetModuleHandleExW(ll::GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS |  ll::GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, func, &mut handle)
    };
    if res == 0 {
        return None;
    }

    let mut path = vec![0u16; MAX_PATH];
    let res = unsafe {
        ll::GetModuleFileNameW(handle, path.as_mut_ptr(), path.len() as u32)
    };
    if res == 0 {
        return None;
    }

    let osstr = wide_from_slice(&path);
    Some(osstr)
}

pub fn fmt_guid(guid: &GUID) -> String {
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
        return None;
    }

    buf.truncate(ret as usize - 1);

    if buf.len() == 0 {
        return None;
    }

    OsString::from_wide(&buf).into_string().ok()
}