use crate::{
    core::{Mat, Size},
    result::Result,
    BorderTypes,
};

mod ffi {
    use crate::{
        core::{MatPointer, Size},
        ffi::FFIResult,
    };

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_gaussian_blur(
            src: *const MatPointer,
            dst: *const MatPointer,
            ksize: Size,
            sigma_x: f64,
            sigma_y: f64,
            border_type: i32,
        ) -> FFIResult<i32>;
    }
}

pub trait GaussianBlur
where
    Self: Sized,
{
    fn gaussian_blur(
        &self,
        ksize: Size,
        sigma_x: f64,
        sigma_y: f64,
        border_type: BorderTypes,
    ) -> Result<Self>;
}

macro_rules! impl_gaussian_blur {
    ($t:ty, $c:tt) => {
        impl GaussianBlur for Mat<$t, $c> {
            fn gaussian_blur(
                &self,
                ksize: Size,
                sigma_x: f64,
                sigma_y: f64,
                border_type: BorderTypes,
            ) -> Result<Self>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_gaussian_blur(
                        self.pointer,
                        dst.pointer,
                        ksize,
                        sigma_x,
                        sigma_y,
                        border_type.bits(),
                    )
                })?;
                Ok(dst)
            }
        }
    };
}

impl_gaussian_blur!(u8, 1);
impl_gaussian_blur!(u8, 2);
impl_gaussian_blur!(u8, 3);
impl_gaussian_blur!(u16, 1);
impl_gaussian_blur!(u16, 2);
impl_gaussian_blur!(u16, 3);
impl_gaussian_blur!(i16, 1);
impl_gaussian_blur!(i16, 2);
impl_gaussian_blur!(i16, 3);
impl_gaussian_blur!(f32, 1);
impl_gaussian_blur!(f32, 2);
impl_gaussian_blur!(f32, 3);
impl_gaussian_blur!(f64, 1);
impl_gaussian_blur!(f64, 2);
impl_gaussian_blur!(f64, 3);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! gaussian_blur_test {
        ($name:ident, $t: ty, $c: tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$t, $c>::from_shape(32, 32).unwrap();
                let ksize = Size {
                    width: 3,
                    height: 3,
                };
                let dst = src
                    .gaussian_blur(ksize, 1., 0., BorderTypes::BORDER_DEFAULT)
                    .unwrap();
                assert_eq!(dst.channels(), $c);
                assert_eq!(dst.rows(), 32);
                assert_eq!(dst.cols(), 32);
            }
        };
    }
    gaussian_blur_test!(gaussian_blur_8uc1_test, u8, 1);
    gaussian_blur_test!(gaussian_blur_8uc2_test, u8, 2);
    gaussian_blur_test!(gaussian_blur_8uc3_test, u8, 3);
    gaussian_blur_test!(gaussian_blur_16uc1_test, u16, 1);
    gaussian_blur_test!(gaussian_blur_16uc2_test, u16, 2);
    gaussian_blur_test!(gaussian_blur_16uc3_test, u16, 3);
    gaussian_blur_test!(gaussian_blur_16sc1_test, i16, 1);
    gaussian_blur_test!(gaussian_blur_16sc2_test, i16, 2);
    gaussian_blur_test!(gaussian_blur_16sc3_test, i16, 3);
    gaussian_blur_test!(gaussian_blur_32fc1_test, f32, 1);
    gaussian_blur_test!(gaussian_blur_32fc2_test, f32, 2);
    gaussian_blur_test!(gaussian_blur_32fc3_test, f32, 3);
    gaussian_blur_test!(gaussian_blur_64fc1_test, f64, 1);
    gaussian_blur_test!(gaussian_blur_64fc2_test, f64, 2);
    gaussian_blur_test!(gaussian_blur_64fc3_test, f64, 3);
}
