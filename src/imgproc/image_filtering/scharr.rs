use crate::{core::Mat, result::Result, BorderTypes, DataTypes};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_scharr(
            src: *const MatPointer,
            dst: *const MatPointer,
            ddepth: i32,
            dx: i32,
            dy: i32,
            scale: f64,
            delta: f64,
            border_type: i32,
        ) -> FFIResult<i32>;
    }
}

pub trait Scharr<T, const C: usize> {
    fn scharr(
        &self,
        dx: i32,
        dy: i32,
        scale: f64,
        delta: f64,
        border_type: BorderTypes,
    ) -> Result<Mat<T, C>>;
}

macro_rules! impl_scharr {
    ($input:ty, $output:ty, $ddepth: expr) => {
        impl<const C: usize> Scharr<$output, C> for Mat<$input, C> {
            fn scharr(
                &self,
                dx: i32,
                dy: i32,
                scale: f64,
                delta: f64,
                border_type: BorderTypes,
            ) -> Result<Mat<$output, C>>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_scharr(
                        self.pointer,
                        dst.pointer,
                        $ddepth,
                        dx,
                        dy,
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

impl_scharr!(u8, u8, DataTypes::CV_8U.bits());
impl_scharr!(u8, i16, DataTypes::CV_16S.bits());
impl_scharr!(u8, f32, DataTypes::CV_32F.bits());
impl_scharr!(u8, f64, DataTypes::CV_64F.bits());

impl_scharr!(u16, u16, DataTypes::CV_16U.bits());
impl_scharr!(u16, f32, DataTypes::CV_32F.bits());
impl_scharr!(u16, f64, DataTypes::CV_64F.bits());

impl_scharr!(i16, i16, DataTypes::CV_16S.bits());
impl_scharr!(i16, f32, DataTypes::CV_32F.bits());
impl_scharr!(i16, f64, DataTypes::CV_64F.bits());

impl_scharr!(f32, f32, DataTypes::CV_32F.bits());

impl_scharr!(f64, f64, DataTypes::CV_64F.bits());

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! scharr_test {
        ($name: ident, $input:ty, $output:ty, $channel:tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$input, $channel>::from_shape(32, 32).unwrap();
                let dst: Mat<$output, $channel> = src
                    .scharr(1, 0, 1., 0., BorderTypes::BORDER_DEFAULT)
                    .unwrap();
                assert_eq!(src.cols(), dst.cols());
                assert_eq!(src.rows(), dst.rows());
                assert_eq!(src.channels(), dst.channels());
            }
        };
    }
    scharr_test!(sep_filter2d_8uc1_to_8uc1_test, u8, u8, 1);
    scharr_test!(sep_filter2d_8uc2_to_8uc2_test, u8, u8, 2);
    scharr_test!(sep_filter2d_8uc3_to_8uc3_test, u8, u8, 3);
    scharr_test!(sep_filter2d_8uc1_to_16s1_test, u8, i16, 1);
    scharr_test!(sep_filter2d_8uc2_to_16s2_test, u8, i16, 2);
    scharr_test!(sep_filter2d_8uc3_to_16s3_test, u8, i16, 3);
    scharr_test!(sep_filter2d_8uc1_to_32fc1_test, u8, f32, 1);
    scharr_test!(sep_filter2d_8uc2_to_32fc2_test, u8, f32, 2);
    scharr_test!(sep_filter2d_8uc3_to_32fc3_test, u8, f32, 3);
    scharr_test!(sep_filter2d_8uc1_to_64fc1_test, u8, f64, 1);
    scharr_test!(sep_filter2d_8uc2_to_64fc2_test, u8, f64, 2);
    scharr_test!(sep_filter2d_8uc3_to_64fc3_test, u8, f64, 3);

    scharr_test!(sep_filter2d_16uc1_to_16uc1_test, u16, u16, 1);
    scharr_test!(sep_filter2d_16uc2_to_16uc2_test, u16, u16, 2);
    scharr_test!(sep_filter2d_16uc3_to_16uc3_test, u16, u16, 3);
    scharr_test!(sep_filter2d_16uc1_to_32fc1_test, u16, f32, 1);
    scharr_test!(sep_filter2d_16uc2_to_32fc2_test, u16, f32, 2);
    scharr_test!(sep_filter2d_16uc3_to_32fc3_test, u16, f32, 3);
    scharr_test!(sep_filter2d_16uc1_to_64fc1_test, u16, f64, 1);
    scharr_test!(sep_filter2d_16uc2_to_64fc2_test, u16, f64, 2);
    scharr_test!(sep_filter2d_16uc3_to_64fc3_test, u16, f64, 3);

    scharr_test!(sep_filter2d_16sc1_to_16sc1_test, i16, i16, 1);
    scharr_test!(sep_filter2d_16sc2_to_16sc2_test, i16, i16, 2);
    scharr_test!(sep_filter2d_16sc3_to_16sc3_test, i16, i16, 3);
    scharr_test!(sep_filter2d_16sc1_to_32fc1_test, i16, f32, 1);
    scharr_test!(sep_filter2d_16sc2_to_32fc2_test, i16, f32, 2);
    scharr_test!(sep_filter2d_16sc3_to_32fc3_test, i16, f32, 3);
    scharr_test!(sep_filter2d_16sc1_to_64fc1_test, i16, f64, 1);
    scharr_test!(sep_filter2d_16sc2_to_64fc2_test, i16, f64, 2);
    scharr_test!(sep_filter2d_16sc3_to_64fc3_test, i16, f64, 3);

    scharr_test!(sep_filter2d_32fc1_to_32fc1_test, f32, f32, 1);
    scharr_test!(sep_filter2d_32fc2_to_32fc2_test, f32, f32, 2);
    scharr_test!(sep_filter2d_32fc3_to_32fc3_test, f32, f32, 3);

    scharr_test!(sep_filter2d_64fc1_to_64fc1_test, f64, f64, 1);
    scharr_test!(sep_filter2d_64fc2_to_64fc2_test, f64, f64, 2);
    scharr_test!(sep_filter2d_64fc3_to_64fc3_test, f64, f64, 3);
}
