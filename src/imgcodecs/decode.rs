use super::consts::ImreadModes;
use crate::core::Mat;

mod ffi {
    use crate::core::MatPointer;

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_imdecode(data: *const u8, size: usize, flags: i32) -> *const MatPointer;
    }
}

pub trait Decode {
    fn decode(data: &[u8]) -> Result<Self, &'static str>
    where
        Self: Sized;
}

impl Decode for Mat<u8, 1> {
    fn decode(data: &[u8]) -> Result<Self, &'static str> {
        let p = unsafe {
            ffi::cv_imdecode(
                data.as_ptr(),
                data.len(),
                ImreadModes::IMREAD_GRAYSCALE.bits(),
            )
        };
        if p.is_null() {
            Err("Pointer is null.")
        } else {
            Ok(Mat::new(p))
        }
    }
}

impl Decode for Mat<u8, 3> {
    fn decode(data: &[u8]) -> Result<Self, &'static str> {
        let p = unsafe {
            ffi::cv_imdecode(data.as_ptr(), data.len(), ImreadModes::IMREAD_COLOR.bits())
        };
        if p.is_null() {
            Err("Pointer is null.")
        } else {
            Ok(Mat::new(p))
        }
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
