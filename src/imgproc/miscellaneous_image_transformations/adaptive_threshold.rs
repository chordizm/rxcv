#![allow(non_upper_case_globals)]
use crate::{core::Mat, result::Result};
use bitflags::bitflags;

bitflags! {
    pub struct AdaptiveThresholdTypes: i32 {
        const ADAPTIVE_THRESH_MEAN_C = 0;
        const ADAPTIVE_THRESH_GAUSSIAN_C = 1;
    }
}

bitflags! {
    pub struct ThresholdTypes: i32 {
        const THRESH_BINARY = 0;
        const THRESH_BINRAY_INV = 1;
    }
}

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_adaptive_threshold(
            src: *const MatPointer,
            dst: *const MatPointer,
            max_value: f64,
            adaptive_method: i32,
            threshold_type: i32,
            block_size: i32,
            c: f64,
        ) -> FFIResult<i32>;
    }
}

pub trait AdaptiveThreshold
where
    Self: Sized,
{
    fn adaptive_threshold(
        &self,
        max_value: f64,
        adaptive_threshold: AdaptiveThresholdTypes,
        threshold_type: ThresholdTypes,
        block_size: i32,
        c: f64,
    ) -> Result<Self>;
}

impl AdaptiveThreshold for Mat<u8, 1> {
    fn adaptive_threshold(
        &self,
        max_value: f64,
        adaptive_method: AdaptiveThresholdTypes,
        threshold_type: ThresholdTypes,
        block_size: i32,
        c: f64,
    ) -> Result<Self> {
        let dst = Mat::<u8, 1>::new()?;
        Result::from(unsafe {
            ffi::cv_adaptive_threshold(
                self.pointer,
                dst.pointer,
                max_value,
                adaptive_method.bits(),
                threshold_type.bits(),
                block_size,
                c,
            )
        })?;
        Ok(dst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adaptive_threshold_test() {
        let src = Mat::<u8, 1>::from_shape(32, 32).unwrap();
        let dst = src
            .adaptive_threshold(
                255.,
                AdaptiveThresholdTypes::ADAPTIVE_THRESH_GAUSSIAN_C,
                ThresholdTypes::THRESH_BINARY,
                3,
                0.,
            )
            .unwrap();
        assert_eq!(src.cols(), dst.cols());
    }

    #[test]
    fn adaptive_threshold_fail_case_test() {
        let src = Mat::<u8, 1>::from_shape(32, 32).unwrap();
        let result = src.adaptive_threshold(
            255.,
            AdaptiveThresholdTypes::ADAPTIVE_THRESH_GAUSSIAN_C,
            ThresholdTypes::THRESH_BINARY,
            2,
            0.,
        );
        assert!(result.is_err());
    }
}
