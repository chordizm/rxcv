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

pub trait BilateralFilter
where
    Self: Sized,
{
    fn bilateral_filter(
        &self,
        d: i32,
        sigma_coilor: f64,
        sigma_space: f64,
        border_type: BorderTypes,
    ) -> Result<Self>;
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
impl_birateral_filter!(u8, 3);
impl_birateral_filter!(f32, 1);
impl_birateral_filter!(f32, 3);

#[cfg(test)]
mod tests {

    use super::*;
    macro_rules! bilateral_test {
        ($name:ident, $t: ty, $c: tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$t, $c>::from_shape(32, 32).unwrap();
                assert_eq!(src.cols(), 32);
                assert_eq!(src.rows(), 32);
                assert_eq!(src.channels(), $c);
                let dst = src
                    .bilateral_filter(15, 20., 20., BorderTypes::BORDER_DEFAULT)
                    .unwrap();
                assert_eq!(src.cols(), dst.cols());
                assert_eq!(src.rows(), dst.rows());
                assert_eq!(src.channels(), dst.channels());
            }
        };
    }

    bilateral_test!(bilateral_8uc1_test, u8, 1);
    bilateral_test!(bilateral_8uc3_test, u8, 3);
    bilateral_test!(bilateral_32fc1_test, f32, 1);
    bilateral_test!(bilateral_32fc3_test, f32, 3);
}
