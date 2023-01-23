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

macro_rules! impl_box_filter {
    ($t:ty, $c:tt) => {
        impl Mat<$t, $c> {
            pub fn box_filter(
                &self,
                r#type: DataTypes,
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
                    ffi::cv_box_filter(
                        self.pointer,
                        dst.pointer,
                        r#type.bits(),
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

impl_box_filter!(u8, 1);
impl_box_filter!(u8, 2);
impl_box_filter!(u8, 3);
impl_box_filter!(u16, 1);
impl_box_filter!(u16, 2);
impl_box_filter!(u16, 3);
impl_box_filter!(i16, 1);
impl_box_filter!(i16, 2);
impl_box_filter!(i16, 3);
impl_box_filter!(i32, 1);
impl_box_filter!(i32, 2);
impl_box_filter!(i32, 3);
impl_box_filter!(f32, 1);
impl_box_filter!(f32, 2);
impl_box_filter!(f32, 3);
impl_box_filter!(f64, 1);
impl_box_filter!(f64, 2);
impl_box_filter!(f64, 3);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! box_filter_test {
        ($name:ident, $t: ty, $c: tt) => {
            #[test]
            fn $name() {
                fn normalize_test() {
                    let src = Mat::<$t, $c>::from_shape(32, 32).unwrap();
                    let ksize = Size {
                        width: 10,
                        height: 10,
                    };
                    let anchor = Point::default();
                    let dst = src
                        .box_filter(
                            DataTypes::CV_8U,
                            ksize,
                            anchor,
                            true,
                            BorderTypes::BORDER_DEFAULT,
                        )
                        .unwrap();
                    assert_eq!(dst.channels(), $c);
                    assert_eq!(dst.rows(), 32);
                    assert_eq!(dst.cols(), 32);
                }
                normalize_test();
                fn none_normalize_test() {
                    let src = Mat::<$t, $c>::from_shape(32, 32).unwrap();
                    let ksize = Size {
                        width: 10,
                        height: 10,
                    };
                    let anchor = Point::default();
                    let dst = src
                        .box_filter(
                            DataTypes::CV_8U,
                            ksize,
                            anchor,
                            true,
                            BorderTypes::BORDER_DEFAULT,
                        )
                        .unwrap();
                    assert_eq!(dst.channels(), $c);
                    assert_eq!(dst.rows(), 32);
                    assert_eq!(dst.cols(), 32);
                }
                none_normalize_test();
            }
        };
    }
    box_filter_test!(box_filter_8uc1_test, u8, 1);
    box_filter_test!(box_filter_8uc2_test, u8, 2);
    box_filter_test!(box_filter_8uc3_test, u8, 3);
    box_filter_test!(box_filter_16uc1_test, u16, 1);
    box_filter_test!(box_filter_16uc2_test, u16, 2);
    box_filter_test!(box_filter_16uc3_test, u16, 3);
    box_filter_test!(box_filter_16sc1_test, i16, 1);
    box_filter_test!(box_filter_16sc2_test, i16, 2);
    box_filter_test!(box_filter_16sc3_test, i16, 3);
    box_filter_test!(box_filter_32sc1_test, i32, 1);
    box_filter_test!(box_filter_32sc2_test, i32, 2);
    box_filter_test!(box_filter_32sc3_test, i32, 3);
    box_filter_test!(box_filter_32fc1_test, f32, 1);
    box_filter_test!(box_filter_32fc2_test, f32, 2);
    box_filter_test!(box_filter_32fc3_test, f32, 3);
    box_filter_test!(box_filter_64fc1_test, f64, 1);
    box_filter_test!(box_filter_64fc2_test, f64, 2);
    box_filter_test!(box_filter_64fc3_test, f64, 3);
}
