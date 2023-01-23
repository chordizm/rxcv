use crate::{
    core::{Mat, Point, Size},
    result::Result,
    BorderTypes,
};

mod ffi {
    use crate::{
        core::{MatPointer, Point, Size},
        ffi::FFIResult,
    };

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_blur(
            src: *const MatPointer,
            dst: *const MatPointer,
            ksize: Size,
            anchor: Point,
            border_type: i32,
        ) -> FFIResult<i32>;
    }
}

pub trait Blur
where
    Self: Sized,
{
    fn blur(&self, ksize: Size, anchor: Point, border_type: BorderTypes) -> Result<Self>;
}

macro_rules! impl_blur {
    ($t:ty, $c:tt) => {
        impl Blur for Mat<$t, $c> {
            fn blur(&self, ksize: Size, anchor: Point, border_type: BorderTypes) -> Result<Self>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_blur(self.pointer, dst.pointer, ksize, anchor, border_type.bits())
                })?;
                Ok(dst)
            }
        }
    };
}

impl_blur!(u8, 1);
impl_blur!(u8, 2);
impl_blur!(u8, 3);
impl_blur!(u16, 1);
impl_blur!(u16, 2);
impl_blur!(u16, 3);
impl_blur!(i16, 1);
impl_blur!(i16, 2);
impl_blur!(i16, 3);
impl_blur!(i32, 1);
impl_blur!(i32, 2);
impl_blur!(i32, 3);
impl_blur!(f32, 1);
impl_blur!(f32, 2);
impl_blur!(f32, 3);
impl_blur!(f64, 1);
impl_blur!(f64, 2);
impl_blur!(f64, 3);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! blur_test {
        ($name:ident, $t: ty, $c: tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$t, $c>::from_shape(32, 32).unwrap();
                let ksize = Size {
                    width: 10,
                    height: 10,
                };
                let anchor = Point::default();
                let dst = src
                    .blur(ksize, anchor, BorderTypes::BORDER_DEFAULT)
                    .unwrap();
                assert_eq!(dst.channels(), $c);
                assert_eq!(dst.rows(), 32);
                assert_eq!(dst.cols(), 32);
            }
        };
    }
    blur_test!(blur_8uc1_test, u8, 1);
    blur_test!(blur_8uc2_test, u8, 2);
    blur_test!(blur_8uc3_test, u8, 3);
    blur_test!(blur_16uc1_test, u16, 1);
    blur_test!(blur_16uc2_test, u16, 2);
    blur_test!(blur_16uc3_test, u16, 3);
    blur_test!(blur_16sc1_test, i16, 1);
    blur_test!(blur_16sc2_test, i16, 2);
    blur_test!(blur_16sc3_test, i16, 3);
    blur_test!(blur_32sc1_test, i32, 1);
    blur_test!(blur_32sc2_test, i32, 2);
    blur_test!(blur_32sc3_test, i32, 3);
    blur_test!(blur_32fc1_test, f32, 1);
    blur_test!(blur_32fc2_test, f32, 2);
    blur_test!(blur_32fc3_test, f32, 3);
    blur_test!(blur_64fc1_test, f64, 1);
    blur_test!(blur_64fc2_test, f64, 2);
    blur_test!(blur_64fc3_test, f64, 3);
}
