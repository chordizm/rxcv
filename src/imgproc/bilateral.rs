use crate::{core::Mat, BorderTypes};

mod ffi {
    use crate::core::MatPointer;

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_bilateral_filter(
            src: *const MatPointer,
            dst: *const MatPointer,
            d: i32,
            sigma_coilor: f64,
            sigma_space: f64,
            border_type: i32,
        ) -> bool;
    }
}

pub trait BilateralFilter {
    fn bilateral_filter(
        &self,
        d: i32,
        sigma_coilor: f64,
        sigma_space: f64,
        border_type: BorderTypes,
    ) -> Result<Self, &'static str>
    where
        Self: Sized;
}

macro_rules! impl_birateral_filter {
    ($t:ty, $c:tt) => {
        impl BilateralFilter for Mat<$t, $c> {
            fn bilateral_filter(
                &self,
                d: i32,
                sigma_coilor: f64,
                sigma_space: f64,
                border_type: BorderTypes,
            ) -> Result<Self, &'static str>
            where
                Self: Sized,
            {
                let dst = Mat::default();
                if unsafe {
                    ffi::cv_bilateral_filter(
                        self.pointer,
                        dst.pointer,
                        d,
                        sigma_coilor,
                        sigma_space,
                        border_type.bits(),
                    )
                } {
                    Ok(dst)
                } else {
                    Err("Failed to Operation")
                }
            }
        }
    };
}

impl_birateral_filter!(u8, 1);
impl_birateral_filter!(u8, 2);
impl_birateral_filter!(u8, 3);
impl_birateral_filter!(f32, 1);
impl_birateral_filter!(f32, 3);

#[cfg(test)]
mod tests {
    use crate::imgcodecs::Read;

    use super::*;

    #[test]
    fn bilateral_filter_8uc3_test() {
        let src = Mat::<u8, 3>::read("mock/lenna.png").unwrap();
        assert_eq!(src.cols(), 512);
        assert_eq!(src.rows(), 512);
        assert_eq!(src.channels(), 3);
        let dst = src
            .bilateral_filter(15, 20., 20., BorderTypes::BORDER_DEFAULT)
            .unwrap();
        assert_eq!(src.cols(), dst.cols());
        assert_eq!(src.rows(), dst.rows());
        assert_eq!(src.channels(), dst.channels());
    }

    #[test]
    fn bilateral_filter_8uc1_test() {
        let src = Mat::<u8, 1>::read("mock/lenna.png").unwrap();
        assert_eq!(src.cols(), 512);
        assert_eq!(src.rows(), 512);
        assert_eq!(src.channels(), 1);
        let dst = src
            .bilateral_filter(15, 20., 20., BorderTypes::BORDER_DEFAULT)
            .unwrap();
        assert_eq!(src.cols(), dst.cols());
        assert_eq!(src.rows(), dst.rows());
        assert_eq!(src.channels(), dst.channels());
    }
}
