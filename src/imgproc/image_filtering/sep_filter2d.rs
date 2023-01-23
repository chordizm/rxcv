use crate::{core::Mat, result::Result, BorderTypes, DataTypes, Point};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult, Point};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_sep_filter2d(
            src: *const MatPointer,
            dst: *const MatPointer,
            ddepth: i32,
            kernel_x: *const MatPointer,
            kernel_y: *const MatPointer,
            anchor: Point,
            delta: f64,
            border_type: i32,
        ) -> FFIResult<i32>;
    }
}

macro_rules! impl_sep_filter2d {
    ($t:ty, $c:tt, $ddepth: expr) => {
        impl Mat<$t, $c> {
            pub fn sep_filter2d(
                &self,
                kernel_x: Mat<f64, 1>,
                kernel_y: Mat<f64, 1>,
                anchor: Point,
                delta: f64,
                border_type: BorderTypes,
            ) -> Result<Self>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_sep_filter2d(
                        self.pointer,
                        dst.pointer,
                        $ddepth,
                        kernel_x.pointer,
                        kernel_y.pointer,
                        anchor,
                        delta,
                        border_type.bits(),
                    )
                })?;
                Ok(dst)
            }
        }
    };
}

impl_sep_filter2d!(u8, 1, DataTypes::CV_8U.bits());
impl_sep_filter2d!(u8, 2, DataTypes::CV_8U.bits());
impl_sep_filter2d!(u8, 3, DataTypes::CV_8U.bits());
impl_sep_filter2d!(u16, 1, DataTypes::CV_16U.bits());
impl_sep_filter2d!(u16, 2, DataTypes::CV_16U.bits());
impl_sep_filter2d!(u16, 3, DataTypes::CV_16U.bits());
impl_sep_filter2d!(i16, 1, DataTypes::CV_16S.bits());
impl_sep_filter2d!(i16, 2, DataTypes::CV_16S.bits());
impl_sep_filter2d!(i16, 3, DataTypes::CV_16S.bits());
impl_sep_filter2d!(f32, 1, DataTypes::CV_32F.bits());
impl_sep_filter2d!(f32, 2, DataTypes::CV_32F.bits());
impl_sep_filter2d!(f32, 3, DataTypes::CV_32F.bits());
impl_sep_filter2d!(f64, 1, DataTypes::CV_64F.bits());
impl_sep_filter2d!(f64, 2, DataTypes::CV_64F.bits());
impl_sep_filter2d!(f64, 3, DataTypes::CV_64F.bits());

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! sep_filter2d_test {
        ($name: ident, $t:ty, $c:tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$t, $c>::from_shape(32, 32).unwrap();
                let data = ndarray::Array::from_shape_vec((3, 1, 1), vec![0., 1., 0.]).unwrap();
                let kernel_x = Mat::<f64, 1>::from_ndarray(&data).unwrap();
                let data = ndarray::Array::from_shape_vec((3, 1, 1), vec![0., 1., 0.]).unwrap();
                let kernel_y = Mat::<f64, 1>::from_ndarray(&data).unwrap();
                let anchor = Point::default();
                let dst = src
                    .sep_filter2d(kernel_x, kernel_y, anchor, 0., BorderTypes::BORDER_DEFAULT)
                    .unwrap();
                assert_eq!(src.cols(), dst.cols());
                assert_eq!(src.rows(), dst.rows());
                assert_eq!(src.channels(), dst.channels());
            }
        };
    }
    sep_filter2d_test!(sep_filter2d_8uc1_test, u8, 1);
    sep_filter2d_test!(sep_filter2d_8uc2_test, u8, 2);
    sep_filter2d_test!(sep_filter2d_8uc3_test, u8, 3);
    sep_filter2d_test!(sep_filter2d_16uc1_test, u16, 1);
    sep_filter2d_test!(sep_filter2d_16uc2_test, u16, 2);
    sep_filter2d_test!(sep_filter2d_16uc3_test, u16, 3);
    sep_filter2d_test!(sep_filter2d_16sc1_test, i16, 1);
    sep_filter2d_test!(sep_filter2d_16sc2_test, i16, 2);
    sep_filter2d_test!(sep_filter2d_16sc3_test, i16, 3);
    sep_filter2d_test!(sep_filter2d_32fc1_test, f32, 1);
    sep_filter2d_test!(sep_filter2d_32fc2_test, f32, 2);
    sep_filter2d_test!(sep_filter2d_32fc3_test, f32, 3);
    sep_filter2d_test!(sep_filter2d_64fc1_test, f64, 1);
    sep_filter2d_test!(sep_filter2d_64fc2_test, f64, 2);
    sep_filter2d_test!(sep_filter2d_64fc3_test, f64, 3);
}
