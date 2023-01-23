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

pub trait SepFilter2d<T, const C: usize> {
    fn sep_filter2d(
        &self,
        kernel_x: Mat<f64, 1>,
        kernel_y: Mat<f64, 1>,
        anchor: Point,
        delta: f64,
        border_type: BorderTypes,
    ) -> Result<Mat<T, C>>;
}

macro_rules! impl_sep_filter2d {
    ($input:ty, $output:ty, $ddepth: expr) => {
        impl<const C: usize> SepFilter2d<$output, C> for Mat<$input, C> {
            fn sep_filter2d(
                &self,
                kernel_x: Mat<f64, 1>,
                kernel_y: Mat<f64, 1>,
                anchor: Point,
                delta: f64,
                border_type: BorderTypes,
            ) -> Result<Mat<$output, C>> {
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

impl_sep_filter2d!(u8, u8, DataTypes::CV_8U.bits());
impl_sep_filter2d!(u8, i16, DataTypes::CV_16S.bits());
impl_sep_filter2d!(u8, f32, DataTypes::CV_32F.bits());
impl_sep_filter2d!(u8, f64, DataTypes::CV_64F.bits());

impl_sep_filter2d!(u16, u16, DataTypes::CV_16U.bits());
impl_sep_filter2d!(u16, f32, DataTypes::CV_32F.bits());
impl_sep_filter2d!(u16, f64, DataTypes::CV_64F.bits());

impl_sep_filter2d!(i16, i16, DataTypes::CV_16S.bits());
impl_sep_filter2d!(i16, f32, DataTypes::CV_32F.bits());
impl_sep_filter2d!(i16, f64, DataTypes::CV_64F.bits());

impl_sep_filter2d!(f32, f32, DataTypes::CV_32F.bits());

impl_sep_filter2d!(f64, f64, DataTypes::CV_64F.bits());

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! sep_filter2d_test {
        ($name: ident, $input:ty, $output:ty, $channel:tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$input, $channel>::from_shape(32, 32).unwrap();
                let data = ndarray::Array::from_shape_vec((3, 1, 1), vec![0., 1., 0.]).unwrap();
                let kernel_x = Mat::<f64, 1>::from_ndarray(&data).unwrap();
                let data = ndarray::Array::from_shape_vec((3, 1, 1), vec![0., 1., 0.]).unwrap();
                let kernel_y = Mat::<f64, 1>::from_ndarray(&data).unwrap();
                let anchor = Point::default();
                let dst: Mat<$output, $channel> = src
                    .sep_filter2d(kernel_x, kernel_y, anchor, 0., BorderTypes::BORDER_DEFAULT)
                    .unwrap();
                assert_eq!(src.cols(), dst.cols());
                assert_eq!(src.rows(), dst.rows());
                assert_eq!(src.channels(), dst.channels());
            }
        };
    }
    sep_filter2d_test!(sep_filter2d_8uc1_to_8uc1_test, u8, u8, 1);
    sep_filter2d_test!(sep_filter2d_8uc2_to_8uc2_test, u8, u8, 2);
    sep_filter2d_test!(sep_filter2d_8uc3_to_8uc3_test, u8, u8, 3);
    sep_filter2d_test!(sep_filter2d_8uc1_to_16s1_test, u8, i16, 1);
    sep_filter2d_test!(sep_filter2d_8uc2_to_16s2_test, u8, i16, 2);
    sep_filter2d_test!(sep_filter2d_8uc3_to_16s3_test, u8, i16, 3);
    sep_filter2d_test!(sep_filter2d_8uc1_to_32fc1_test, u8, f32, 1);
    sep_filter2d_test!(sep_filter2d_8uc2_to_32fc2_test, u8, f32, 2);
    sep_filter2d_test!(sep_filter2d_8uc3_to_32fc3_test, u8, f32, 3);
    sep_filter2d_test!(sep_filter2d_8uc1_to_64fc1_test, u8, f64, 1);
    sep_filter2d_test!(sep_filter2d_8uc2_to_64fc2_test, u8, f64, 2);
    sep_filter2d_test!(sep_filter2d_8uc3_to_64fc3_test, u8, f64, 3);

    sep_filter2d_test!(sep_filter2d_16uc1_to_16uc1_test, u16, u16, 1);
    sep_filter2d_test!(sep_filter2d_16uc2_to_16uc2_test, u16, u16, 2);
    sep_filter2d_test!(sep_filter2d_16uc3_to_16uc3_test, u16, u16, 3);
    sep_filter2d_test!(sep_filter2d_16uc1_to_32fc1_test, u16, f32, 1);
    sep_filter2d_test!(sep_filter2d_16uc2_to_32fc2_test, u16, f32, 2);
    sep_filter2d_test!(sep_filter2d_16uc3_to_32fc3_test, u16, f32, 3);
    sep_filter2d_test!(sep_filter2d_16uc1_to_64fc1_test, u16, f64, 1);
    sep_filter2d_test!(sep_filter2d_16uc2_to_64fc2_test, u16, f64, 2);
    sep_filter2d_test!(sep_filter2d_16uc3_to_64fc3_test, u16, f64, 3);

    sep_filter2d_test!(sep_filter2d_16sc1_to_16sc1_test, i16, i16, 1);
    sep_filter2d_test!(sep_filter2d_16sc2_to_16sc2_test, i16, i16, 2);
    sep_filter2d_test!(sep_filter2d_16sc3_to_16sc3_test, i16, i16, 3);
    sep_filter2d_test!(sep_filter2d_16sc1_to_32fc1_test, i16, f32, 1);
    sep_filter2d_test!(sep_filter2d_16sc2_to_32fc2_test, i16, f32, 2);
    sep_filter2d_test!(sep_filter2d_16sc3_to_32fc3_test, i16, f32, 3);
    sep_filter2d_test!(sep_filter2d_16sc1_to_64fc1_test, i16, f64, 1);
    sep_filter2d_test!(sep_filter2d_16sc2_to_64fc2_test, i16, f64, 2);
    sep_filter2d_test!(sep_filter2d_16sc3_to_64fc3_test, i16, f64, 3);

    sep_filter2d_test!(sep_filter2d_32fc1_to_32fc1_test, f32, f32, 1);
    sep_filter2d_test!(sep_filter2d_32fc2_to_32fc2_test, f32, f32, 2);
    sep_filter2d_test!(sep_filter2d_32fc3_to_32fc3_test, f32, f32, 3);

    sep_filter2d_test!(sep_filter2d_64fc1_to_64fc1_test, f64, f64, 1);
    sep_filter2d_test!(sep_filter2d_64fc2_to_64fc2_test, f64, f64, 2);
    sep_filter2d_test!(sep_filter2d_64fc3_to_64fc3_test, f64, f64, 3);
}
