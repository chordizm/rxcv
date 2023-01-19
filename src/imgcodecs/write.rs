use crate::core::Mat;
use std::ffi::CString;

mod ffi {
    use crate::core::MatPointer;
    use std::ffi::c_char;

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_imwrite(img: *const MatPointer, path: *const c_char) -> bool;
    }
}

pub trait Write {
    fn write(&self, path: &str) -> Result<(), &'static str>;
}

macro_rules! impl_imwrite {
    ($t:ty, $c:tt) => {
        impl Write for Mat<$t, $c> {
            fn write(&self, path: &str) -> Result<(), &'static str> {
                let path = CString::new(path).unwrap();
                let path = path.as_ptr();
                if unsafe { ffi::cv_imwrite(self.pointer, path) } {
                    Ok(())
                } else {
                    Err("Failed to operation.")
                }
            }
        }
    };
}

impl_imwrite!(u8, 1);
impl_imwrite!(u8, 3);
