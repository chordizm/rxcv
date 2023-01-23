use crate::{core::Mat, result::Result, BorderTypes, DataTypes};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_filter2d(
            src: *const MatPointer,
            dst: *const MatPointer,
            ddepth: i32,
            kernel: *const MatPointer,
            anchor_x: i32,
            anchor_y: i32,
            delta: f64,
            border_type: i32,
        ) -> FFIResult<i32>;
    }
}

macro_rules! impl_filter2d {
    ($t:ty, $c:tt, $ddepth:expr) => {
        impl Mat<$t, $c> {
            pub fn filter2d(
                &self,
                kernel: Mat<f64, 1>,
                anchor_x: i32,
                anchor_y: i32,
                delta: f64,
                border_type: BorderTypes,
            ) -> Result<Self>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_filter2d(
                        self.pointer,
                        dst.pointer,
                        $ddepth,
                        kernel.pointer,
                        anchor_x,
                        anchor_y,
                        delta,
                        border_type.bits(),
                    )
                })?;
                Ok(dst)
            }
        }
    };
}

impl_filter2d!(u8, 1, DataTypes::CV_8UC1.bits());
impl_filter2d!(u8, 3, DataTypes::CV_8UC3.bits());
impl_filter2d!(f64, 1, DataTypes::CV_64FC1.bits());
impl_filter2d!(f64, 3, DataTypes::CV_64FC3.bits());

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imgcodecs::Read;

    #[test]
    fn filter2d_8uc1_test() {
        let src = Mat::<u8, 1>::read("mock/lenna.png").unwrap();
        assert_eq!(src.cols(), 512);
        assert_eq!(src.rows(), 512);
        assert_eq!(src.channels(), 1);
        let data =
            ndarray::Array::from_shape_vec((3, 3, 1), vec![0., 1., 0., 1., -4., 1., 0., 1., 0.])
                .unwrap();
        let kernel = Mat::<f64, 1>::from_ndarray(&data).unwrap();
        let dst = src
            .filter2d(kernel, -1, -1, 0., BorderTypes::BORDER_DEFAULT)
            .unwrap();
        assert_eq!(src.cols(), dst.cols());
        assert_eq!(src.rows(), dst.rows());
        assert_eq!(src.channels(), dst.channels());
    }
    #[test]
    fn filter2d_8uc3_test() {
        let src = Mat::<u8, 3>::read("mock/lenna.png").unwrap();
        assert_eq!(src.cols(), 512);
        assert_eq!(src.rows(), 512);
        assert_eq!(src.channels(), 3);
        let data =
            ndarray::Array::from_shape_vec((3, 3, 1), vec![0., 1., 0., 1., -4., 1., 0., 1., 0.])
                .unwrap();
        let kernel = Mat::<f64, 1>::from_ndarray(&data).unwrap();
        let dst = src
            .filter2d(kernel, -1, -1, 0., BorderTypes::BORDER_DEFAULT)
            .unwrap();
        assert_eq!(src.cols(), dst.cols());
        assert_eq!(src.rows(), dst.rows());
        assert_eq!(src.channels(), dst.channels());
    }
}
