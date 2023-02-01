use crate::{core::Mat, result::Result};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_invert_affine_transform(
            src: *const MatPointer,
            dst: *const MatPointer,
        ) -> FFIResult<i32>;
    }
}

pub trait InvertAffineTransform
where
    Self: Sized,
{
    fn invert_affine_transform(&self) -> Result<Self>;
}

macro_rules! impl_invert_affine_transform {
    ($t:ty, $c:tt) => {
        impl InvertAffineTransform for Mat<$t, $c> {
            fn invert_affine_transform(&self) -> Result<Self>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_invert_affine_transform(self.pointer, dst.pointer)
                })?;
                Ok(dst)
            }
        }
    };
}

impl_invert_affine_transform!(f32, 1);
impl_invert_affine_transform!(f64, 1);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! invert_affine_transform_test {
        ($name:ident, $t: ty, $c: tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$t, $c>::from_shape(2, 3).unwrap();
                assert_eq!(src.cols(), 3);
                assert_eq!(src.rows(), 2);
                let dst = src.invert_affine_transform().unwrap();
                assert_eq!(dst.channels(), $c);
                assert_eq!(dst.cols(), 3);
                assert_eq!(dst.rows(), 2);
            }
        };
    }
    invert_affine_transform_test!(invert_affine_transform_32fc1_test, f32, 1);
    invert_affine_transform_test!(invert_affine_transform_64fc1_test, f64, 1);
}
