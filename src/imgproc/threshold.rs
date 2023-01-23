//! This module provide cv::threshold
use super::consts::ThresholdTypes;
use crate::{core::Mat, result::Result};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_threshold(
            src: *const MatPointer,
            dst: *const MatPointer,
            thresh: i32,
            maxval: i32,
            r#type: i32,
        ) -> FFIResult<f64>;
    }
}

impl<T> Mat<T, 1> {
    //! Single channel Mat can use threshold method.
    pub fn threshold(
        &self,
        thresh: i32,
        max_value: i32,
        r#type: ThresholdTypes,
    ) -> Result<(f64, Self)>
    where
        Self: Sized,
    {
        let dst = Mat::new()?;
        let value = Result::from(unsafe {
            ffi::cv_threshold(self.pointer, dst.pointer, thresh, max_value, r#type.bits())
        })?;
        Ok((value, dst))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threshold_test() {
        let src = Mat::mock_7x6_square_5x4().cvt_color_bgr2gray().unwrap();
        let (thresh, dst) = src
            .threshold(
                0,
                255,
                ThresholdTypes::THRESH_BINARY | ThresholdTypes::THRESH_OTSU,
            )
            .unwrap();
        assert_eq!(thresh, 0.0);
        assert_eq!(dst.channels(), 1);
        assert_eq!(dst.data(), src.data());
    }
}
