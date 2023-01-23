use crate::{core::Mat, result::Result, BorderTypes, DataTypes};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_sobel(
            src: *const MatPointer,
            dst: *const MatPointer,
            ddepth: i32,
            dx: i32,
            dy: i32,
            ksize: i32,
            scale: f64,
            delta: f64,
            border_type: i32,
        ) -> FFIResult<i32>;
    }
}

macro_rules! impl_sobel {
    ($t:ty, $c:tt, $ddepth: expr) => {
        impl Mat<$t, $c> {
            pub fn sobel(
                &self,
                dx: i32,
                dy: i32,
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
                    ffi::cv_sobel(
                        self.pointer,
                        dst.pointer,
                        $ddepth,
                        dx,
                        dy,
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

impl_sobel!(u8, 1, DataTypes::CV_8U.bits());
impl_sobel!(u8, 2, DataTypes::CV_8U.bits());
impl_sobel!(u8, 3, DataTypes::CV_8U.bits());
impl_sobel!(u16, 1, DataTypes::CV_16U.bits());
impl_sobel!(u16, 2, DataTypes::CV_16U.bits());
impl_sobel!(u16, 3, DataTypes::CV_16U.bits());
impl_sobel!(i16, 1, DataTypes::CV_16S.bits());
impl_sobel!(i16, 2, DataTypes::CV_16S.bits());
impl_sobel!(i16, 3, DataTypes::CV_16S.bits());
impl_sobel!(f32, 1, DataTypes::CV_32F.bits());
impl_sobel!(f32, 2, DataTypes::CV_32F.bits());
impl_sobel!(f32, 3, DataTypes::CV_32F.bits());
impl_sobel!(f64, 1, DataTypes::CV_64F.bits());
impl_sobel!(f64, 2, DataTypes::CV_64F.bits());
impl_sobel!(f64, 3, DataTypes::CV_64F.bits());

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! sobel_test {
        ($name: ident, $t:ty, $c:tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$t, $c>::from_shape(32, 32).unwrap();
                let dst = src
                    .sobel(1, 1, 3, 1., 0., BorderTypes::BORDER_DEFAULT)
                    .unwrap();
                assert_eq!(src.cols(), dst.cols());
                assert_eq!(src.rows(), dst.rows());
                assert_eq!(src.channels(), dst.channels());
            }
        };
    }
    sobel_test!(sobel_8uc1_test, u8, 1);
    sobel_test!(sobel_8uc2_test, u8, 2);
    sobel_test!(sobel_8uc3_test, u8, 3);
    sobel_test!(sobel_16uc1_test, u16, 1);
    sobel_test!(sobel_16uc2_test, u16, 2);
    sobel_test!(sobel_16uc3_test, u16, 3);
    sobel_test!(sobel_16sc1_test, i16, 1);
    sobel_test!(sobel_16sc2_test, i16, 2);
    sobel_test!(sobel_16sc3_test, i16, 3);
    sobel_test!(sobel_32fc1_test, f32, 1);
    sobel_test!(sobel_32fc2_test, f32, 2);
    sobel_test!(sobel_32fc3_test, f32, 3);
    sobel_test!(sobel_64fc1_test, f64, 1);
    sobel_test!(sobel_64fc2_test, f64, 2);
    sobel_test!(sobel_64fc3_test, f64, 3);
}
