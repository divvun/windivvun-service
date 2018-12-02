#![allow(non_camel_case_types, non_snake_case, unused, clippy::unreadable_literal)]

use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::UINT;
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, LPDISPATCH, VARIANT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl, LPUNKNOWN};

include!(concat!(env!("OUT_DIR"), "/spellcheckprovider.rs"));
