use crate::{core::Mat, result::Result, BorderTypes};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_spatial_gradient(
            src: *const MatPointer,
            dx: *const MatPointer,
            dy: *const MatPointer,
            ksize: i32,
            border_type: i32,
        ) -> FFIResult<i32>;
    }
}

macro_rules! impl_spatial_gradient {
    ($t:ty, $c:tt, $ddepth: expr) => {
        impl Mat<$t, $c> {
            pub fn spatial_gradient(
                &self,
                ksize: i32,
                border_type: BorderTypes,
            ) -> Result<(Self, Self)>
            where
                Self: Sized,
            {
                let dx = Mat::new()?;
                let dy = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_spatial_gradient(
                        self.pointer,
                        dx.pointer,
                        dy.pointer,
                        ksize,
                        border_type.bits(),
                    )
                })?;
                Ok((dx, dy))
            }
        }
    };
}

impl_spatial_gradient!(u8, 1, DataTypes::CV_8U.bits());

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! spatial_gradient_test {
        ($name: ident, $t:ty, $c:tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$t, $c>::from_shape(32, 32).unwrap();
                let (dx, dy) = src
                    .spatial_gradient(3, BorderTypes::BORDER_DEFAULT)
                    .unwrap();
                assert_eq!(src.cols(), dx.cols());
                assert_eq!(src.rows(), dx.rows());
                assert_eq!(src.channels(), dx.channels());
                assert_eq!(src.cols(), dy.cols());
                assert_eq!(src.rows(), dy.rows());
                assert_eq!(src.channels(), dy.channels());
            }
        };
    }
    spatial_gradient_test!(spatial_gradient_8uc1_test, u8, 1);
}
