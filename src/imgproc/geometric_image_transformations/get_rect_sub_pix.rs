use crate::{core::Mat, result::Result, DataTypes, Point2f, Size};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult, Point2f, Size};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_get_rect_sub_pix(
            src: *const MatPointer,
            patch_size: Size,
            center: Point2f,
            patch: *const MatPointer,
            patch_type: i32,
        ) -> FFIResult<i32>;
    }
}

pub trait GetRectSubPix<Depth, const C: usize>
where
    Self: Sized,
{
    fn get_rect_sub_pix(&self, patch_size: Size, center: Point2f) -> Result<Mat<Depth, C>>;
}

macro_rules! impl_get_rect_sub_pix {
    ($input:ty, $channel:tt, $output:ty, $code:expr) => {
        impl GetRectSubPix<$output, $channel> for Mat<$input, $channel> {
            fn get_rect_sub_pix(
                &self,
                patch_size: Size,
                center: Point2f,
            ) -> Result<Mat<$output, $channel>> {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_get_rect_sub_pix(self.pointer, patch_size, center, dst.pointer, $code)
                })?;
                Ok(dst)
            }
        }
    };
}

macro_rules! impl_get_rect_sub_pix_all_depth {
    ($t:ty, $channel:tt) => {
        impl_get_rect_sub_pix!($t, $channel, u8, DataTypes::CV_8U.bits());
        // impl_get_rect_sub_pix!($t, $channel, i8, DataTypes::CV_8S.bits());
        // impl_get_rect_sub_pix!($t, $channel, u16, DataTypes::CV_16U.bits());
        // impl_get_rect_sub_pix!($t, $channel, i16, DataTypes::CV_16S.bits());
        // impl_get_rect_sub_pix!($t, $channel, i32, DataTypes::CV_32S.bits());
        impl_get_rect_sub_pix!($t, $channel, f32, DataTypes::CV_32F.bits());
        // impl_get_rect_sub_pix!($t, $channel, f64, DataTypes::CV_64F.bits());
    };
}

macro_rules! impl_get_rect_sub_pix_all {
    ($t:ty) => {
        impl_get_rect_sub_pix_all_depth!($t, 1);
        // impl_get_rect_sub_pix_all_depth!($t, 2);
        impl_get_rect_sub_pix_all_depth!($t, 3);
    };
}

impl_get_rect_sub_pix_all!(u8);
impl_get_rect_sub_pix!(f32, 1, f32, DataTypes::CV_32F.bits());
impl_get_rect_sub_pix!(f32, 3, f32, DataTypes::CV_32F.bits());
// impl_get_rect_sub_pix_all!(i8);
// impl_get_rect_sub_pix_all!(u16);
// impl_get_rect_sub_pix_all!(i16);
// impl_get_rect_sub_pix_all!(i32);
// impl_get_rect_sub_pix_all!(f32);
// impl_get_rect_sub_pix_all!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! get_rect_sub_pix_test {
        ($name:ident, $input: ty, $output: ty) => {
            #[test]
            fn $name() {
                let src = Mat::<$input, 1>::from_shape(1024, 1024).unwrap();
                let patch_size = Size {
                    width: 32,
                    height: 32,
                };
                let center = Point2f { x: 512., y: 512. };
                let dst: Mat<$output, 1> = src.get_rect_sub_pix(patch_size, center).unwrap();
                assert_eq!(dst.channels(), 1);
                assert_eq!(dst.cols(), 32);
                assert_eq!(dst.rows(), 32);
                // let src = Mat::<$input, 2>::from_shape(1024, 1024).unwrap();
                // let patch_size = Size {
                //     width: 32,
                //     height: 32,
                // };
                // let center = Point2f { x: 512., y: 512. };
                // let dst: Mat<$output, 2> = src.get_rect_sub_pix(patch_size, center).unwrap();
                // assert_eq!(dst.channels(), 2);
                // assert_eq!(dst.cols(), 32);
                // assert_eq!(dst.rows(), 32);
                let src = Mat::<$input, 3>::from_shape(1024, 1024).unwrap();
                let patch_size = Size {
                    width: 32,
                    height: 32,
                };
                let center = Point2f { x: 512., y: 512. };
                let dst: Mat<$output, 3> = src.get_rect_sub_pix(patch_size, center).unwrap();
                assert_eq!(dst.channels(), 3);
                assert_eq!(dst.cols(), 32);
                assert_eq!(dst.rows(), 32);
            }
        };
    }
    get_rect_sub_pix_test!(get_rect_sub_pix_8u_8u_test, u8, u8);
    // get_rect_sub_pix_test!(get_rect_sub_pix_8u_8s_test, u8, i8);
    // get_rect_sub_pix_test!(get_rect_sub_pix_8u_16u_test, u8, u16);
    // get_rect_sub_pix_test!(get_rect_sub_pix_8u_16s_test, u8, i16);
    // get_rect_sub_pix_test!(get_rect_sub_pix_8u_32s_test, u8, i32);
    get_rect_sub_pix_test!(get_rect_sub_pix_8u_32f_test, u8, f32);
    // get_rect_sub_pix_test!(get_rect_sub_pix_8u_64f_test, u8, f64);
    // get_rect_sub_pix_test!(get_rect_sub_pix_8s_8u_test, i8, u8);
    // get_rect_sub_pix_test!(get_rect_sub_pix_8s_8s_test, i8, i8);
    // get_rect_sub_pix_test!(get_rect_sub_pix_8s_16u_test, i8, u16);
    // get_rect_sub_pix_test!(get_rect_sub_pix_8s_16s_test, i8, i16);
    // get_rect_sub_pix_test!(get_rect_sub_pix_8s_32s_test, i8, i32);
    // get_rect_sub_pix_test!(get_rect_sub_pix_8s_32f_test, i8, f32);
    // get_rect_sub_pix_test!(get_rect_sub_pix_8s_64f_test, i8, f64);

    // get_rect_sub_pix_test!(get_rect_sub_pix_16u_8u_test, u16, u8);
    // get_rect_sub_pix_test!(get_rect_sub_pix_16u_8s_test, u16, i8);
    // get_rect_sub_pix_test!(get_rect_sub_pix_16u_16u_test, u16, u16);
    // get_rect_sub_pix_test!(get_rect_sub_pix_16u_16s_test, u16, i16);
    // get_rect_sub_pix_test!(get_rect_sub_pix_16u_32s_test, u16, i32);
    // get_rect_sub_pix_test!(get_rect_sub_pix_16u_32f_test, u16, f32);
    // get_rect_sub_pix_test!(get_rect_sub_pix_16u_64f_test, u16, f64);
    // get_rect_sub_pix_test!(get_rect_sub_pix_16s_8u_test, i16, u8);
    // get_rect_sub_pix_test!(get_rect_sub_pix_16s_8s_test, i16, i8);
    // get_rect_sub_pix_test!(get_rect_sub_pix_16s_16u_test, i16, u16);
    // get_rect_sub_pix_test!(get_rect_sub_pix_16s_16s_test, i16, i16);
    // get_rect_sub_pix_test!(get_rect_sub_pix_16s_32s_test, i16, i32);
    // get_rect_sub_pix_test!(get_rect_sub_pix_16s_32f_test, i16, f32);
    // get_rect_sub_pix_test!(get_rect_sub_pix_16s_64f_test, i16, f64);

    // get_rect_sub_pix_test!(get_rect_sub_pix_32s_8u_test, i32, u8);
    // get_rect_sub_pix_test!(get_rect_sub_pix_32s_8s_test, i32, i8);
    // get_rect_sub_pix_test!(get_rect_sub_pix_32s_16u_test, i32, u16);
    // get_rect_sub_pix_test!(get_rect_sub_pix_32s_16s_test, i32, i16);
    // get_rect_sub_pix_test!(get_rect_sub_pix_32s_32s_test, i32, i32);
    // get_rect_sub_pix_test!(get_rect_sub_pix_32s_32f_test, i32, f32);
    // get_rect_sub_pix_test!(get_rect_sub_pix_32s_64f_test, i32, f64);

    // get_rect_sub_pix_test!(get_rect_sub_pix_32f_8u_test, f32, u8);
    // get_rect_sub_pix_test!(get_rect_sub_pix_32f_8s_test, f32, i8);
    // get_rect_sub_pix_test!(get_rect_sub_pix_32f_16u_test, f32, u16);
    // get_rect_sub_pix_test!(get_rect_sub_pix_32f_16s_test, f32, i16);
    // get_rect_sub_pix_test!(get_rect_sub_pix_32f_32s_test, f32, i32);
    get_rect_sub_pix_test!(get_rect_sub_pix_32f_32f_test, f32, f32);
    // get_rect_sub_pix_test!(get_rect_sub_pix_32f_64f_test, f32, f64);

    // get_rect_sub_pix_test!(get_rect_sub_pix_64f_8u_test, f64, u8);
    // get_rect_sub_pix_test!(get_rect_sub_pix_64f_8s_test, f64, i8);
    // get_rect_sub_pix_test!(get_rect_sub_pix_64f_16u_test, f64, u16);
    // get_rect_sub_pix_test!(get_rect_sub_pix_64f_16s_test, f64, i16);
    // get_rect_sub_pix_test!(get_rect_sub_pix_64f_32s_test, f64, i32);
    // get_rect_sub_pix_test!(get_rect_sub_pix_64f_32f_test, f64, f32);
    // get_rect_sub_pix_test!(get_rect_sub_pix_64f_64f_test, f64, f64);
}
