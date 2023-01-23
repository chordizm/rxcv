use crate::{core::Mat, result::Result, BorderTypes};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_bilateral_filter(
            src: *const MatPointer,
            dst: *const MatPointer,
            d: i32,
            sigma_coilor: f64,
            sigma_space: f64,
            border_type: i32,
        ) -> FFIResult<i32>;
    }
}

macro_rules! impl_birateral_filter {
    ($t:ty, $c:tt) => {
        impl Mat<$t, $c> {
            pub fn bilateral_filter(
                &self,
                d: i32,
                sigma_coilor: f64,
                sigma_space: f64,
                border_type: BorderTypes,
            ) -> Result<Self>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_bilateral_filter(
                        self.pointer,
                        dst.pointer,
                        d,
                        sigma_coilor,
                        sigma_space,
                        border_type.bits(),
                    )
                })?;
                Ok(dst)
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
