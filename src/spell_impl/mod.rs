macro_rules! IMPL_UNKNOWN {
  ($x:ty, $y:ty) => {
    #[implementation(IUnknown)]
    impl $y {
        fn QueryInterface(&mut self, riid: &GUID, obj: &mut usize) -> HRESULT {
            use winapi::shared::winerror::{E_NOTIMPL, S_OK};
            use winapi::Interface;

            *obj = 0;

            info!("QueryInterface {}", ::util::fmt_guid(riid));
            if IsEqualGUID(riid, &<$x>::uuidof()) || IsEqualGUID(riid, &IUnknown::uuidof()) {
                *obj = self as *mut _ as usize;
                self.AddRef();
                S_OK
            } else {
                E_NOTIMPL
            }
        }

        fn AddRef(&mut self) -> ULONG {
            let prev = self.refs.fetch_add(1, Ordering::SeqCst);
            info!("AddRef: {}", prev);
            prev + 1
        }

        fn Release(&mut self) -> ULONG {
            let prev = self.refs.fetch_sub(1, Ordering::SeqCst);
            info!("Release: {}", prev);
            if prev == 1 {
                let _box = unsafe { Box::from_raw(self as *mut _) };
            }
            prev - 1
        }
    }
  }
}

pub mod SpellCheckProviderFactory;
pub mod ClassFactory;
pub mod EnumString;
pub mod SpellCheckProvider;