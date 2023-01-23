use crate::{
    core::{Mat, Point, Size},
    result::Result,
    BorderTypes, DataTypes,
};

mod ffi {
    use crate::{
        core::{MatPointer, Point, Size},
        ffi::FFIResult,
    };

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_sqr_box_filter(
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

macro_rules! impl_sqr_box_filter {
    ($t:ty, $c:tt, $ddepth:expr) => {
        impl Mat<$t, $c> {
            pub fn sqr_box_filter(
                &self,
                ksize: Size,
                anchor: Point,
                normalize: bool,
                border_type: BorderTypes,
            ) -> Result<Self>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_sqr_box_filter(
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

impl_sqr_box_filter!(u8, 1, DataTypes::CV_8U.bits());
impl_sqr_box_filter!(u8, 2, DataTypes::CV_8U.bits());
impl_sqr_box_filter!(u8, 3, DataTypes::CV_8U.bits());
impl_sqr_box_filter!(u16, 1, DataTypes::CV_16U.bits());
impl_sqr_box_filter!(u16, 2, DataTypes::CV_16U.bits());
impl_sqr_box_filter!(u16, 3, DataTypes::CV_16U.bits());
impl_sqr_box_filter!(i16, 1, DataTypes::CV_16S.bits());
impl_sqr_box_filter!(i16, 2, DataTypes::CV_16S.bits());
impl_sqr_box_filter!(i16, 3, DataTypes::CV_16S.bits());
impl_sqr_box_filter!(f32, 1, DataTypes::CV_32F.bits());
impl_sqr_box_filter!(f32, 2, DataTypes::CV_32F.bits());
impl_sqr_box_filter!(f32, 3, DataTypes::CV_32F.bits());
impl_sqr_box_filter!(f64, 1, DataTypes::CV_64F.bits());
impl_sqr_box_filter!(f64, 2, DataTypes::CV_64F.bits());
impl_sqr_box_filter!(f64, 3, DataTypes::CV_64F.bits());

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! sqr_box_filter_test {
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
                    .sqr_box_filter(ksize, anchor, true, BorderTypes::BORDER_DEFAULT)
                    .unwrap();
                assert_eq!(dst.channels(), $c);
                assert_eq!(dst.rows(), 32);
                assert_eq!(dst.cols(), 32);
                let ksize = Size {
                    width: 10,
                    height: 10,
                };
                let anchor = Point::default();
                let dst = src
                    .sqr_box_filter(ksize, anchor, false, BorderTypes::BORDER_DEFAULT)
                    .unwrap();
                assert_eq!(dst.channels(), $c);
                assert_eq!(dst.rows(), 32);
                assert_eq!(dst.cols(), 32);
            }
        };
    }
    sqr_box_filter_test!(sqr_box_filter_8uc1_test, u8, 1);
    sqr_box_filter_test!(sqr_box_filter_8uc2_test, u8, 2);
    sqr_box_filter_test!(sqr_box_filter_8uc3_test, u8, 3);
    sqr_box_filter_test!(sqr_box_filter_16uc1_test, u16, 1);
    sqr_box_filter_test!(sqr_box_filter_16uc2_test, u16, 2);
    sqr_box_filter_test!(sqr_box_filter_16uc3_test, u16, 3);
    sqr_box_filter_test!(sqr_box_filter_16sc1_test, i16, 1);
    sqr_box_filter_test!(sqr_box_filter_16sc2_test, i16, 2);
    sqr_box_filter_test!(sqr_box_filter_16sc3_test, i16, 3);
    sqr_box_filter_test!(sqr_box_filter_32fc1_test, f32, 1);
    sqr_box_filter_test!(sqr_box_filter_32fc2_test, f32, 2);
    sqr_box_filter_test!(sqr_box_filter_32fc3_test, f32, 3);
    sqr_box_filter_test!(sqr_box_filter_64fc1_test, f64, 1);
    sqr_box_filter_test!(sqr_box_filter_64fc2_test, f64, 2);
    sqr_box_filter_test!(sqr_box_filter_64fc3_test, f64, 3);
}
