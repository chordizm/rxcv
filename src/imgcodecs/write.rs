use crate::{core::Mat, result::Result};
use std::ffi::CString;

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};
    use std::ffi::c_char;

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_imwrite(img: *const MatPointer, path: *const c_char) -> FFIResult<()>;
    }
}

pub trait Write {
    fn write(&self, path: &str) -> Result<()>;
}

macro_rules! impl_imwrite {
    ($t:ty, $c:tt) => {
        impl Write for Mat<$t, $c> {
            fn write(&self, path: &str) -> Result<()> {
                let path = CString::new(path).unwrap();
                let path = path.as_ptr();
                Result::from(unsafe { ffi::cv_imwrite(self.pointer, path) })
            }
        }
    };
}

impl_imwrite!(u8, 1);
impl_imwrite!(u8, 3);
