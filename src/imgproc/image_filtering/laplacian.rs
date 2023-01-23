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

pub trait Laplacian<T, const C: usize> {
    fn laplacian(
        &self,
        ksize: i32,
        scale: f64,
        delta: f64,
        border_type: BorderTypes,
    ) -> Result<Mat<T, C>>;
}

macro_rules! impl_laplacian {
    ($input:ty, $output:ty, $ddepth: expr) => {
        impl<const C: usize> Laplacian<$output, C> for Mat<$input, C> {
            fn laplacian(
                &self,
                ksize: i32,
                scale: f64,
                delta: f64,
                border_type: BorderTypes,
            ) -> Result<Mat<$output, C>>
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

impl_laplacian!(u8, u8, DataTypes::CV_8U.bits());
impl_laplacian!(u8, i16, DataTypes::CV_16S.bits());
impl_laplacian!(u8, f32, DataTypes::CV_32F.bits());
impl_laplacian!(u8, f64, DataTypes::CV_64F.bits());

impl_laplacian!(u16, u16, DataTypes::CV_16U.bits());
impl_laplacian!(u16, f32, DataTypes::CV_32F.bits());
impl_laplacian!(u16, f64, DataTypes::CV_64F.bits());

impl_laplacian!(i16, i16, DataTypes::CV_16S.bits());
impl_laplacian!(i16, f32, DataTypes::CV_32F.bits());
impl_laplacian!(i16, f64, DataTypes::CV_64F.bits());

impl_laplacian!(f32, f32, DataTypes::CV_32F.bits());

impl_laplacian!(f64, f64, DataTypes::CV_64F.bits());

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! laplacian_test {
        ($name: ident, $input:ty, $output:ty, $channel:tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$input, $channel>::from_shape(32, 32).unwrap();
                let dst: Mat<$output, $channel> = src
                    .laplacian(1, 1., 0., BorderTypes::BORDER_DEFAULT)
                    .unwrap();
                assert_eq!(dst.channels(), $channel);
                assert_eq!(dst.rows(), 32);
                assert_eq!(dst.cols(), 32);
            }
        };
    }
    laplacian_test!(laplacian_8uc1_to_8uc1_test, u8, u8, 1);
    laplacian_test!(laplacian_8uc2_to_8uc2_test, u8, u8, 2);
    laplacian_test!(laplacian_8uc3_to_8uc3_test, u8, u8, 3);
    laplacian_test!(laplacian_8uc1_to_16s1_test, u8, i16, 1);
    laplacian_test!(laplacian_8uc2_to_16s2_test, u8, i16, 2);
    laplacian_test!(laplacian_8uc3_to_16s3_test, u8, i16, 3);
    laplacian_test!(laplacian_8uc1_to_32fc1_test, u8, f32, 1);
    laplacian_test!(laplacian_8uc2_to_32fc2_test, u8, f32, 2);
    laplacian_test!(laplacian_8uc3_to_32fc3_test, u8, f32, 3);
    laplacian_test!(laplacian_8uc1_to_64fc1_test, u8, f64, 1);
    laplacian_test!(laplacian_8uc2_to_64fc2_test, u8, f64, 2);
    laplacian_test!(laplacian_8uc3_to_64fc3_test, u8, f64, 3);

    laplacian_test!(laplacian_16uc1_to_16uc1_test, u16, u16, 1);
    laplacian_test!(laplacian_16uc2_to_16uc2_test, u16, u16, 2);
    laplacian_test!(laplacian_16uc3_to_16uc3_test, u16, u16, 3);
    laplacian_test!(laplacian_16uc1_to_32fc1_test, u16, f32, 1);
    laplacian_test!(laplacian_16uc2_to_32fc2_test, u16, f32, 2);
    laplacian_test!(laplacian_16uc3_to_32fc3_test, u16, f32, 3);
    laplacian_test!(laplacian_16uc1_to_64fc1_test, u16, f64, 1);
    laplacian_test!(laplacian_16uc2_to_64fc2_test, u16, f64, 2);
    laplacian_test!(laplacian_16uc3_to_64fc3_test, u16, f64, 3);

    laplacian_test!(laplacian_16sc1_to_16sc1_test, i16, i16, 1);
    laplacian_test!(laplacian_16sc2_to_16sc2_test, i16, i16, 2);
    laplacian_test!(laplacian_16sc3_to_16sc3_test, i16, i16, 3);
    laplacian_test!(laplacian_16sc1_to_32fc1_test, i16, f32, 1);
    laplacian_test!(laplacian_16sc2_to_32fc2_test, i16, f32, 2);
    laplacian_test!(laplacian_16sc3_to_32fc3_test, i16, f32, 3);
    laplacian_test!(laplacian_16sc1_to_64fc1_test, i16, f64, 1);
    laplacian_test!(laplacian_16sc2_to_64fc2_test, i16, f64, 2);
    laplacian_test!(laplacian_16sc3_to_64fc3_test, i16, f64, 3);

    laplacian_test!(laplacian_32fc1_to_32fc1_test, f32, f32, 1);
    laplacian_test!(laplacian_32fc2_to_32fc2_test, f32, f32, 2);
    laplacian_test!(laplacian_32fc3_to_32fc3_test, f32, f32, 3);

    laplacian_test!(laplacian_64fc1_to_64fc1_test, f64, f64, 1);
    laplacian_test!(laplacian_64fc2_to_64fc2_test, f64, f64, 2);
    laplacian_test!(laplacian_64fc3_to_64fc3_test, f64, f64, 3);
}
