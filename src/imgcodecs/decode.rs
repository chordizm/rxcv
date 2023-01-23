use super::consts::ImreadModes;
use crate::{core::Mat, result::Result};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_imdecode(
            data: *const u8,
            size: usize,
            flags: i32,
        ) -> FFIResult<*const MatPointer>;
    }
}

impl Mat<u8, 1> {
    pub fn decode(data: &[u8]) -> Result<Self> {
        let pointer = Result::from(unsafe {
            ffi::cv_imdecode(
                data.as_ptr(),
                data.len(),
                ImreadModes::IMREAD_GRAYSCALE.bits(),
            )
        })?;
        Ok(Mat::from_ptr(pointer))
    }
}

impl Mat<u8, 3> {
    pub fn decode(data: &[u8]) -> Result<Self> {
        let pointer = Result::from(unsafe {
            ffi::cv_imdecode(data.as_ptr(), data.len(), ImreadModes::IMREAD_COLOR.bits())
        })?;
        Ok(Mat::from_ptr(pointer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imdecode_test() {
        let data: &[u8] = include_bytes!("../../mock/7x6_white.png");
        let src = Mat::<u8, 3>::decode(data).unwrap();
        assert_eq!(src.cols(), 7);
        assert_eq!(src.rows(), 6);
        assert_eq!(src.channels(), 3);
        assert_eq!(src.size(), 7 * 6 * 3);
        assert_eq!(src.data().len(), 7 * 6 * 3);
        assert!(src.data().iter().all(|&value| value == 255));
    }
}
