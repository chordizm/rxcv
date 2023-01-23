use crate::{
    core::{DataTypes, Mat, Point, Size},
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
        pub(super) fn cv_box_filter(
            src: *const MatPointer,
            dst: *const MatPointer,
            ddepth: i32,
            ksize: Size,
            anchor: Point,
            normalize: bool,
            border_type: i32,
        ) -> FFIResult<i32>;
    }
}

pub trait BoxFilter<T, const C: usize> {
    fn box_filter(
        &self,
        ksize: Size,
        anchor: Point,
        normalize: bool,
        border_type: BorderTypes,
    ) -> Result<Mat<T, C>>;
}

macro_rules! impl_box_filter {
    ($input:ty, $output:ty, $ddepth: expr) => {
        impl<const C: usize> BoxFilter<$output, C> for Mat<$input, C> {
            fn box_filter(
                &self,
                ksize: Size,
                anchor: Point,
                normalize: bool,
                border_type: BorderTypes,
            ) -> Result<Mat<$output, C>>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_box_filter(
                        self.pointer,
                        dst.pointer,
                        $ddepth,
                        ksize,
                        anchor,
                        normalize,
                        border_type.bits(),
                    )
                })?;
                Ok(dst)
            }
        }
    };
}

impl_box_filter!(u8, u8, DataTypes::CV_8U.bits());
impl_box_filter!(u8, i16, DataTypes::CV_16S.bits());
impl_box_filter!(u8, f32, DataTypes::CV_32F.bits());
impl_box_filter!(u8, f64, DataTypes::CV_64F.bits());

impl_box_filter!(u16, u16, DataTypes::CV_16U.bits());
impl_box_filter!(u16, f32, DataTypes::CV_32F.bits());
impl_box_filter!(u16, f64, DataTypes::CV_64F.bits());

impl_box_filter!(i16, i16, DataTypes::CV_16S.bits());
impl_box_filter!(i16, f32, DataTypes::CV_32F.bits());
impl_box_filter!(i16, f64, DataTypes::CV_64F.bits());

impl_box_filter!(f32, f32, DataTypes::CV_32F.bits());

impl_box_filter!(f64, f64, DataTypes::CV_64F.bits());

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! box_filter_test {
        ($name: ident, $input:ty, $output:ty, $channel:tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$input, $channel>::from_shape(32, 32).unwrap();
                let ksize = Size {
                    width: 10,
                    height: 10,
                };
                let anchor = Point::default();
                let dst: Mat<$output, $channel> = src
                    .box_filter(ksize, anchor, true, BorderTypes::BORDER_DEFAULT)
                    .unwrap();
                assert_eq!(dst.channels(), $channel);
                assert_eq!(dst.rows(), 32);
                assert_eq!(dst.cols(), 32);
            }
        };
    }
    box_filter_test!(box_filter_8uc1_to_8uc1_test, u8, u8, 1);
    box_filter_test!(box_filter_8uc2_to_8uc2_test, u8, u8, 2);
    box_filter_test!(box_filter_8uc3_to_8uc3_test, u8, u8, 3);
    box_filter_test!(box_filter_8uc1_to_16s1_test, u8, i16, 1);
    box_filter_test!(box_filter_8uc2_to_16s2_test, u8, i16, 2);
    box_filter_test!(box_filter_8uc3_to_16s3_test, u8, i16, 3);
    box_filter_test!(box_filter_8uc1_to_32fc1_test, u8, f32, 1);
    box_filter_test!(box_filter_8uc2_to_32fc2_test, u8, f32, 2);
    box_filter_test!(box_filter_8uc3_to_32fc3_test, u8, f32, 3);
    box_filter_test!(box_filter_8uc1_to_64fc1_test, u8, f64, 1);
    box_filter_test!(box_filter_8uc2_to_64fc2_test, u8, f64, 2);
    box_filter_test!(box_filter_8uc3_to_64fc3_test, u8, f64, 3);

    box_filter_test!(box_filter_16uc1_to_16uc1_test, u16, u16, 1);
    box_filter_test!(box_filter_16uc2_to_16uc2_test, u16, u16, 2);
    box_filter_test!(box_filter_16uc3_to_16uc3_test, u16, u16, 3);
    box_filter_test!(box_filter_16uc1_to_32fc1_test, u16, f32, 1);
    box_filter_test!(box_filter_16uc2_to_32fc2_test, u16, f32, 2);
    box_filter_test!(box_filter_16uc3_to_32fc3_test, u16, f32, 3);
    box_filter_test!(box_filter_16uc1_to_64fc1_test, u16, f64, 1);
    box_filter_test!(box_filter_16uc2_to_64fc2_test, u16, f64, 2);
    box_filter_test!(box_filter_16uc3_to_64fc3_test, u16, f64, 3);

    box_filter_test!(box_filter_16sc1_to_16sc1_test, i16, i16, 1);
    box_filter_test!(box_filter_16sc2_to_16sc2_test, i16, i16, 2);
    box_filter_test!(box_filter_16sc3_to_16sc3_test, i16, i16, 3);
    box_filter_test!(box_filter_16sc1_to_32fc1_test, i16, f32, 1);
    box_filter_test!(box_filter_16sc2_to_32fc2_test, i16, f32, 2);
    box_filter_test!(box_filter_16sc3_to_32fc3_test, i16, f32, 3);
    box_filter_test!(box_filter_16sc1_to_64fc1_test, i16, f64, 1);
    box_filter_test!(box_filter_16sc2_to_64fc2_test, i16, f64, 2);
    box_filter_test!(box_filter_16sc3_to_64fc3_test, i16, f64, 3);

    box_filter_test!(box_filter_32fc1_to_32fc1_test, f32, f32, 1);
    box_filter_test!(box_filter_32fc2_to_32fc2_test, f32, f32, 2);
    box_filter_test!(box_filter_32fc3_to_32fc3_test, f32, f32, 3);

    box_filter_test!(box_filter_64fc1_to_64fc1_test, f64, f64, 1);
    box_filter_test!(box_filter_64fc2_to_64fc2_test, f64, f64, 2);
    box_filter_test!(box_filter_64fc3_to_64fc3_test, f64, f64, 3);
}
