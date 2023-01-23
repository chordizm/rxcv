use crate::{
    core::{DataTypes, Mat},
    result::Result,
    BorderTypes,
};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_laplacian(
            src: *const MatPointer,
            dst: *const MatPointer,
            ddepth: i32,
            ksize: i32,
            scale: f64,
            delta: f64,
            border_type: i32,
        ) -> FFIResult<i32>;
    }
}

macro_rules! impl_laplacian {
    ($t:ty, $c:tt, $ddepth: expr) => {
        impl Mat<$t, $c> {
            pub fn laplacian(
                &self,
                ksize: i32,
                scale: f64,
                delta: f64,
                border_type: BorderTypes,
            ) -> Result<Self>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_laplacian(
                        self.pointer,
                        dst.pointer,
                        $ddepth,
                        ksize,
                        scale,
                        delta,
                        border_type.bits(),
                    )
                })?;
                Ok(dst)
            }
        }
    };
}

impl_laplacian!(u8, 1, DataTypes::CV_8U.bits());
impl_laplacian!(u8, 2, DataTypes::CV_8U.bits());
impl_laplacian!(u8, 3, DataTypes::CV_8U.bits());
impl_laplacian!(u16, 1, DataTypes::CV_16U.bits());
impl_laplacian!(u16, 2, DataTypes::CV_16U.bits());
impl_laplacian!(u16, 3, DataTypes::CV_16U.bits());
impl_laplacian!(i16, 1, DataTypes::CV_16S.bits());
impl_laplacian!(i16, 2, DataTypes::CV_16S.bits());
impl_laplacian!(i16, 3, DataTypes::CV_16S.bits());
impl_laplacian!(f32, 1, DataTypes::CV_32F.bits());
impl_laplacian!(f32, 2, DataTypes::CV_32F.bits());
impl_laplacian!(f32, 3, DataTypes::CV_32F.bits());
impl_laplacian!(f64, 1, DataTypes::CV_64F.bits());
impl_laplacian!(f64, 2, DataTypes::CV_64F.bits());
impl_laplacian!(f64, 3, DataTypes::CV_64F.bits());

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! laplacian_test {
        ($name:ident, $t: ty, $c: tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$t, $c>::from_shape(32, 32).unwrap();
                let dst = src
                    .laplacian(1, 1., 0., BorderTypes::BORDER_DEFAULT)
                    .unwrap();
                assert_eq!(dst.channels(), $c);
                assert_eq!(dst.rows(), 32);
                assert_eq!(dst.cols(), 32);
            }
        };
    }
    laplacian_test!(laplacian_8uc1_test, u8, 1);
    laplacian_test!(laplacian_8uc2_test, u8, 2);
    laplacian_test!(laplacian_8uc3_test, u8, 3);
    laplacian_test!(laplacian_16uc1_test, u16, 1);
    laplacian_test!(laplacian_16uc2_test, u16, 2);
    laplacian_test!(laplacian_16uc3_test, u16, 3);
    laplacian_test!(laplacian_16sc1_test, i16, 1);
    laplacian_test!(laplacian_16sc2_test, i16, 2);
    laplacian_test!(laplacian_16sc3_test, i16, 3);
    laplacian_test!(laplacian_32fc1_test, f32, 1);
    laplacian_test!(laplacian_32fc2_test, f32, 2);
    laplacian_test!(laplacian_32fc3_test, f32, 3);
    laplacian_test!(laplacian_64fc1_test, f64, 1);
    laplacian_test!(laplacian_64fc2_test, f64, 2);
    laplacian_test!(laplacian_64fc3_test, f64, 3);
}
