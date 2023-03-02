use crate::{core::Mat, result::Result};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_blend_linear(
            src1: *const MatPointer,
            src2: *const MatPointer,
            weights1: *const MatPointer,
            weights2: *const MatPointer,
            dst: *const MatPointer,
        ) -> FFIResult<i32>;
    }
}

pub trait BlendLinear
where
    Self: Sized,
{
    fn blend_linear(
        &self,
        src: &Self,
        weights1: &Mat<f32, 1>,
        weights2: &Mat<f32, 1>,
    ) -> Result<Self>;
}

macro_rules! impl_blend_linear {
    ($t:ty, $c:tt) => {
        impl BlendLinear for Mat<$t, $c> {
            fn blend_linear(
                &self,
                src: &Self,
                weights1: &Mat<f32, 1>,
                weights2: &Mat<f32, 1>,
            ) -> Result<Self>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_blend_linear(
                        self.pointer,
                        src.pointer,
                        weights1.pointer,
                        weights2.pointer,
                        dst.pointer,
                    )
                })?;
                Ok(dst)
            }
        }
    };
}

impl_blend_linear!(u8, 1);
impl_blend_linear!(u8, 2);
impl_blend_linear!(u8, 3);
impl_blend_linear!(f32, 1);
impl_blend_linear!(f32, 2);
impl_blend_linear!(f32, 3);

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! blend_linear_test {
        ($name: ident, $t:ty, $c:tt) => {
            #[test]
            fn $name() {
                let src1 = Mat::<$t, $c>::from_shape(32, 32).unwrap();
                let src2 = Mat::<$t, $c>::from_shape(32, 32).unwrap();
                let weights1 = Mat::<f32, 1>::from_shape(32, 32).unwrap();
                let weights2 = Mat::<f32, 1>::from_shape(32, 32).unwrap();
                let dst = src1.blend_linear(&src2, &weights1, &weights2).unwrap();
                assert_eq!(src1.cols(), dst.cols());
                assert_eq!(src1.rows(), dst.rows());
                assert_eq!(src1.channels(), dst.channels());
            }
        };
    }
    blend_linear_test!(blend_linear_8uc1_test, u8, 1);
    blend_linear_test!(blend_linear_8uc2_test, u8, 2);
    blend_linear_test!(blend_linear_8uc3_test, u8, 3);
    blend_linear_test!(blend_linear_32fc1_test, f32, 1);
    blend_linear_test!(blend_linear_32fc2_test, f32, 2);
    blend_linear_test!(blend_linear_32fc3_test, f32, 3);
}
