#![cfg(windows)] 

use winapi::um::winnt::{LPCWSTR, HRESULT};

use winapi::shared::ntdef::ULONG;
use winapi::shared::winerror::{S_OK, E_INVALIDARG, E_POINTER, E_NOINTERFACE, S_FALSE};
use winapi::shared::guiddef::{IsEqualGUID, GUID, REFIID};
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{BOOL, TRUE};
use winapi::Interface;

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::objidlbase::{IEnumString, IEnumStringVtbl};
use winapi::um::combaseapi::CoTaskMemAlloc;
use winapi::shared::wtypesbase::{LPOLESTR, OLECHAR};

use std::sync::atomic::{AtomicU32, Ordering};

use com_impl::{ComInterface, interface, implementation};

use std::slice::Iter;
use std::vec::Vec;
use std::mem;
use ::util;

#[interface(IEnumString)]
pub struct EnumString {
    refs: AtomicU32,
    values: Vec<String>,
    offset: usize
}

#[implementation(IUnknown)]
impl EnumString {
    fn QueryInterface(&mut self, riid: &GUID, obj: &mut usize) -> HRESULT {
        use winapi::shared::winerror::{E_NOTIMPL, S_OK};
        use winapi::Interface;

        *obj = 0;

        if IsEqualGUID(riid, &IEnumString::uuidof()) || IsEqualGUID(riid, &IUnknown::uuidof()) {
            *obj = self as *mut _ as usize;
            self.AddRef();
            S_OK
        } else {
            E_NOTIMPL
        }
    }

    fn AddRef(&mut self) -> ULONG {
        let prev = self.refs.fetch_add(1, Ordering::SeqCst);
        prev + 1
    }

    fn Release(&mut self) -> ULONG {
        let prev = self.refs.fetch_sub(1, Ordering::SeqCst);
        if prev == 1 {
            let _box = unsafe { Box::from_raw(self as *mut _) };
        }
        prev - 1
    }
}

#[implementation(IEnumString)]
impl EnumString {
    fn Next(&mut self, celt: ULONG, rgelt: *mut LPOLESTR, pceltFetched: *mut ULONG) -> HRESULT {
        info!("Next for {} values", celt);

        let values = self.values.iter().skip(self.offset).take(celt as usize).collect::<Vec<&String>>();

        info!("{} values fetched", values.len());

        if values.len() == 0 {
            return S_FALSE;
        }

        self.offset = self.offset + values.len();

        unsafe {
            // Allocate pointer list
            let ptr_list_size = mem::size_of::<&[OLECHAR]>() * values.len();
            let ptr_list = CoTaskMemAlloc(ptr_list_size) as *mut LPOLESTR;

            info!("Allocated ptrs");

            for (i, item) in values.iter().enumerate() {
                info!("Str {} => {}", i, item);
                let elem = util::to_u16s(item).unwrap();
                let elem_str = CoTaskMemAlloc(elem.len() * mem::size_of::<OLECHAR>()) as *mut OLECHAR;
                // Copy string
                for (j, c) in elem.iter().enumerate() {
                    *elem_str.offset(j as isize) = *c;
                }
                *ptr_list.offset(i as isize) = elem_str;
            }
        }

        S_OK
    }

    fn Skip(&mut self, celt: ULONG) -> HRESULT {
        self.offset = self.offset +  celt as usize;
        S_OK
    }
    
    fn Reset(&mut self) -> HRESULT {
        self.offset = 0;
        S_OK
    }

    fn Clone(&mut self, ppenum: *mut *mut IEnumString) -> HRESULT {
        unsafe {
            let mut val = EnumString::new_with_offset(self.values.clone(), self.offset);
            *ppenum = val as *mut _;
        }
        S_OK
    }
}

impl EnumString {
    pub fn new(values: Vec<String>) -> *mut EnumString {
        Self::new_with_offset(values, 0)
    }

    pub fn new_with_offset(values: Vec<String>, offset: usize) -> *mut EnumString {
        let s = Self {
            __vtable: Box::new(Self::create_vtable()),
            refs: AtomicU32::new(1),
            values,
            offset
        };

        let ptr = Box::into_raw(Box::new(s));

        ptr as *mut _
    }
}