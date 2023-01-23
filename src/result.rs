use crate::ffi::FFIResult;

pub(crate) type Result<T> = std::result::Result<T, &'static str>;

impl<T> From<FFIResult<T>> for Result<T> {
    fn from(value: FFIResult<T>) -> Self {
        if value.error.is_null() {
            Ok(value.ok)
        } else {
            match unsafe { std::ffi::CStr::from_ptr(value.error) }.to_str() {
                Ok(msg) => Err(msg),
                Err(_) => Err(""),
            }
        }
    }
}
