use crate::{
    core::{Mat, Point},
    result::Result,
    BorderTypes,
};

mod ffi {
    use crate::{
        core::{MatPointer, Point},
        ffi::FFIResult,
    };

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_erode(
            src: *const MatPointer,
            dst: *const MatPointer,
            kernel: *const MatPointer,
            anchor: Point,
            iterations: i32,
            border_type: i32,
        ) -> FFIResult<i32>;
    }
}

pub trait Erode
where
    Self: Sized,
{
    fn erode(
        &self,
        kernel: Mat<u8, 1>,
        anchor: Point,
        iterations: i32,
        border_type: BorderTypes,
    ) -> Result<Self>;
}

macro_rules! impl_erode {
    ($t:ty, $c:tt) => {
        impl Erode for Mat<$t, $c> {
            fn erode(
                &self,
                kernel: Mat<u8, 1>,
                anchor: Point,
                iterations: i32,
                border_type: BorderTypes,
            ) -> Result<Self>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_erode(
                        self.pointer,
                        dst.pointer,
                        kernel.pointer,
                        anchor,
                        iterations,
                        border_type.bits(),
                    )
                })?;
                Ok(dst)
            }
        }
    };
}

impl_erode!(u8, 1);
impl_erode!(u8, 2);
impl_erode!(u8, 3);
impl_erode!(u16, 1);
impl_erode!(u16, 2);
impl_erode!(u16, 3);
impl_erode!(i16, 1);
impl_erode!(i16, 2);
impl_erode!(i16, 3);
impl_erode!(f32, 1);
impl_erode!(f32, 2);
impl_erode!(f32, 3);
impl_erode!(f64, 1);
impl_erode!(f64, 2);
impl_erode!(f64, 3);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! erode_test {
        ($name:ident, $t: ty, $c: tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$t, $c>::from_shape(32, 32).unwrap();
                let data =
                    ndarray::Array::from_shape_vec((3, 3, 1), vec![0, 1, 0, 1, 1, 1, 0, 1, 0])
                        .unwrap();
                let kernel = Mat::<u8, 1>::from_ndarray(&data).unwrap();
                let anchor = Point::default();
                let dst = src
                    .erode(kernel, anchor, 1, BorderTypes::BORDER_DEFAULT)
                    .unwrap();
                assert_eq!(dst.channels(), $c);
                assert_eq!(dst.rows(), 32);
                assert_eq!(dst.cols(), 32);
            }
        };
    }
    erode_test!(erode_8uc1_test, u8, 1);
    erode_test!(erode_8uc2_test, u8, 2);
    erode_test!(erode_8uc3_test, u8, 3);
    erode_test!(erode_16uc1_test, u16, 1);
    erode_test!(erode_16uc2_test, u16, 2);
    erode_test!(erode_16uc3_test, u16, 3);
    erode_test!(erode_16sc1_test, i16, 1);
    erode_test!(erode_16sc2_test, i16, 2);
    erode_test!(erode_16sc3_test, i16, 3);
    erode_test!(erode_32fc1_test, f32, 1);
    erode_test!(erode_32fc2_test, f32, 2);
    erode_test!(erode_32fc3_test, f32, 3);
    erode_test!(erode_64fc1_test, f64, 1);
    erode_test!(erode_64fc2_test, f64, 2);
    erode_test!(erode_64fc3_test, f64, 3);
}
