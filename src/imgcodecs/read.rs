use super::consts::ImreadModes;
use crate::{core::Mat, result::Result};
use std::ffi::CString;

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};
    use std::ffi::c_char;

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_imread(path: *const c_char, flags: i32) -> FFIResult<*const MatPointer>;
    }
}

pub trait Read {
    fn read(path: &str) -> Result<Self>
    where
        Self: Sized;
}

impl Read for Mat<u8, 1> {
    fn read(path: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let path = CString::new(path).unwrap();
        let path = path.as_ptr();
        let pointer =
            Result::from(unsafe { ffi::cv_imread(path, ImreadModes::IMREAD_GRAYSCALE.bits()) })?;
        Ok(Mat::from_ptr(pointer))
    }
}

impl Read for Mat<u8, 3> {
    fn read(path: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let path = CString::new(path).unwrap();
        let path = path.as_ptr();
        let pointer =
            Result::from(unsafe { ffi::cv_imread(path, ImreadModes::IMREAD_COLOR.bits()) })?;
        Ok(Mat::from_ptr(pointer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imdecode_test() {
        let src = Mat::<u8, 3>::read("mock/7x6_white.png").unwrap();
        assert_eq!(src.cols(), 7);
        assert_eq!(src.rows(), 6);
        assert_eq!(src.channels(), 3);
        assert_eq!(src.size(), 7 * 6 * 3);
        assert_eq!(src.data().len(), 7 * 6 * 3);
        assert!(src.data().iter().all(|&value| value == 255));
    }
}
