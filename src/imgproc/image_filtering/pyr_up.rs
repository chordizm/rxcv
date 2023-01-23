use crate::{core::Mat, result::Result, BorderTypes, Size};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult, Size};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_pyr_up(
            src: *const MatPointer,
            dst: *const MatPointer,
            dstsize: Size,
            border_type: i32,
        ) -> FFIResult<i32>;
    }
}

macro_rules! impl_pyr_up {
    ($t:ty, $c:tt) => {
        impl Mat<$t, $c> {
            pub fn pyr_up(&self, dstsize: Size, border_type: BorderTypes) -> Result<Self>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_pyr_up(self.pointer, dst.pointer, dstsize, border_type.bits())
                })?;
                Ok(dst)
            }
        }
    };
}

impl_pyr_up!(u8, 1);
impl_pyr_up!(u8, 2);
impl_pyr_up!(u8, 3);
impl_pyr_up!(u16, 1);
impl_pyr_up!(u16, 2);
impl_pyr_up!(u16, 3);
impl_pyr_up!(i16, 1);
impl_pyr_up!(i16, 2);
impl_pyr_up!(i16, 3);
impl_pyr_up!(f32, 1);
impl_pyr_up!(f32, 2);
impl_pyr_up!(f32, 3);
impl_pyr_up!(f64, 1);
impl_pyr_up!(f64, 2);
impl_pyr_up!(f64, 3);

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! pyr_up_test {
        ($name: ident, $t:ty, $c:tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$t, $c>::from_shape(32, 32).unwrap();
                let dstsize = Size {
                    width: src.cols() * 2,
                    height: src.rows() * 2,
                };
                let dst = src.pyr_up(dstsize, BorderTypes::BORDER_DEFAULT).unwrap();
                assert_eq!(src.cols() * 2, dst.cols());
                assert_eq!(src.rows() * 2, dst.rows());
                assert_eq!(src.channels(), dst.channels());
            }
        };
    }
    pyr_up_test!(pyr_up_8uc1_test, u8, 1);
    pyr_up_test!(pyr_up_8uc2_test, u8, 2);
    pyr_up_test!(pyr_up_8uc3_test, u8, 3);
    pyr_up_test!(pyr_up_16uc1_test, u16, 1);
    pyr_up_test!(pyr_up_16uc2_test, u16, 2);
    pyr_up_test!(pyr_up_16uc3_test, u16, 3);
    pyr_up_test!(pyr_up_16sc1_test, i16, 1);
    pyr_up_test!(pyr_up_16sc2_test, i16, 2);
    pyr_up_test!(pyr_up_16sc3_test, i16, 3);
    pyr_up_test!(pyr_up_32fc1_test, f32, 1);
    pyr_up_test!(pyr_up_32fc2_test, f32, 2);
    pyr_up_test!(pyr_up_32fc3_test, f32, 3);
    pyr_up_test!(pyr_up_64fc1_test, f64, 1);
    pyr_up_test!(pyr_up_64fc2_test, f64, 2);
    pyr_up_test!(pyr_up_64fc3_test, f64, 3);
}
