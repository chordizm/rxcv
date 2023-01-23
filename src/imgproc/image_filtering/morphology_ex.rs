use crate::{
    core::{Mat, Point},
    imgproc::MorphTypes,
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
        pub(super) fn cv_morphology_ex(
            src: *const MatPointer,
            dst: *const MatPointer,
            op: i32,
            kernel: *const MatPointer,
            anchor: Point,
            iterations: i32,
            border_type: i32,
        ) -> FFIResult<i32>;
    }
}

pub trait MorphologyEx
where
    Self: Sized,
{
    fn morphology_ex(
        &self,
        op: MorphTypes,
        kernel: Mat<u8, 1>,
        anchor: Point,
        iterations: i32,
        border_type: BorderTypes,
    ) -> Result<Self>;
}

macro_rules! impl_morphology_ex {
    ($t:ty, $c:tt) => {
        impl MorphologyEx for Mat<$t, $c> {
            fn morphology_ex(
                &self,
                op: MorphTypes,
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
                    ffi::cv_morphology_ex(
                        self.pointer,
                        dst.pointer,
                        op.bits(),
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

impl_morphology_ex!(u8, 1);
impl_morphology_ex!(u8, 2);
impl_morphology_ex!(u8, 3);
impl_morphology_ex!(u16, 1);
impl_morphology_ex!(u16, 2);
impl_morphology_ex!(u16, 3);
impl_morphology_ex!(i16, 1);
impl_morphology_ex!(i16, 2);
impl_morphology_ex!(i16, 3);
impl_morphology_ex!(f32, 1);
impl_morphology_ex!(f32, 2);
impl_morphology_ex!(f32, 3);
impl_morphology_ex!(f64, 1);
impl_morphology_ex!(f64, 2);
impl_morphology_ex!(f64, 3);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! morphology_ex_test {
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
                    .morphology_ex(
                        MorphTypes::MORPH_OPEN,
                        kernel,
                        anchor,
                        1,
                        BorderTypes::BORDER_DEFAULT,
                    )
                    .unwrap();
                assert_eq!(dst.channels(), $c);
                assert_eq!(dst.rows(), 32);
                assert_eq!(dst.cols(), 32);
            }
        };
    }
    morphology_ex_test!(morphology_ex_8uc1_test, u8, 1);
    morphology_ex_test!(morphology_ex_8uc2_test, u8, 2);
    morphology_ex_test!(morphology_ex_8uc3_test, u8, 3);
    morphology_ex_test!(morphology_ex_16uc1_test, u16, 1);
    morphology_ex_test!(morphology_ex_16uc2_test, u16, 2);
    morphology_ex_test!(morphology_ex_16uc3_test, u16, 3);
    morphology_ex_test!(morphology_ex_16sc1_test, i16, 1);
    morphology_ex_test!(morphology_ex_16sc2_test, i16, 2);
    morphology_ex_test!(morphology_ex_16sc3_test, i16, 3);
    morphology_ex_test!(morphology_ex_32fc1_test, f32, 1);
    morphology_ex_test!(morphology_ex_32fc2_test, f32, 2);
    morphology_ex_test!(morphology_ex_32fc3_test, f32, 3);
    morphology_ex_test!(morphology_ex_64fc1_test, f64, 1);
    morphology_ex_test!(morphology_ex_64fc2_test, f64, 2);
    morphology_ex_test!(morphology_ex_64fc3_test, f64, 3);
}
