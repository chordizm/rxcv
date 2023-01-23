use crate::{core::Mat, result::Result};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_median_blur(
            src: *const MatPointer,
            dst: *const MatPointer,
            ksize: i32,
        ) -> FFIResult<i32>;
    }
}

macro_rules! impl_median_blur {
    ($t:ty, $c:tt) => {
        impl Mat<$t, $c> {
            pub fn median_blur(&self, ksize: i32) -> Result<Self>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe { ffi::cv_median_blur(self.pointer, dst.pointer, ksize) })?;
                Ok(dst)
            }
        }
    };
}

impl_median_blur!(u8, 1);
impl_median_blur!(u8, 2);
impl_median_blur!(u8, 3);
impl_median_blur!(u16, 1);
impl_median_blur!(u16, 2);
impl_median_blur!(u16, 3);
impl_median_blur!(i16, 1);
impl_median_blur!(i16, 2);
impl_median_blur!(i16, 3);
impl_median_blur!(f32, 1);
impl_median_blur!(f32, 2);
impl_median_blur!(f32, 3);

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! median_blur_test {
        ($name: ident, $t:ty, $c:tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$t, $c>::from_shape(32, 32).unwrap();
                let dst = src.median_blur(5).unwrap();
                assert_eq!(src.cols(), dst.cols());
                assert_eq!(src.rows(), dst.rows());
                assert_eq!(src.channels(), dst.channels());
            }
        };
    }
    median_blur_test!(median_blur_8uc1_test, u8, 1);
    median_blur_test!(median_blur_8uc2_test, u8, 2);
    median_blur_test!(median_blur_8uc3_test, u8, 3);
    median_blur_test!(median_blur_16uc1_test, u16, 1);
    median_blur_test!(median_blur_16uc2_test, u16, 2);
    median_blur_test!(median_blur_16uc3_test, u16, 3);
    median_blur_test!(median_blur_16sc1_test, i16, 1);
    median_blur_test!(median_blur_16sc2_test, i16, 2);
    median_blur_test!(median_blur_16sc3_test, i16, 3);
    median_blur_test!(median_blur_32fc1_test, f32, 1);
    median_blur_test!(median_blur_32fc2_test, f32, 2);
    median_blur_test!(median_blur_32fc3_test, f32, 3);
}
