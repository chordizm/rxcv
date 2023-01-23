use crate::{core::Mat, result::Result};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_median_blur(
            src: *const MatPointer,
            dst: *const MatPointer,
            ksize: i32,
        ) -> FFIResult<i32>;
    }
}

impl<T, const C: usize> Mat<T, C> {
    pub fn median_blur(&self, ksize: i32) -> Result<Self>
    where
        Self: Sized,
    {
        let dst = Mat::new()?;
        Result::from(unsafe { ffi::cv_median_blur(self.pointer, dst.pointer, ksize) })?;
        Ok(dst)
    }
    pub fn median_blur3x3(&self) -> Result<Self>
    where
        Self: Sized,
    {
        self.median_blur(3)
    }
    pub fn median_blur5x5(&self) -> Result<Self>
    where
        Self: Sized,
    {
        self.median_blur(5)
    }
    pub fn median_blur7x7(&self) -> Result<Self>
    where
        Self: Sized,
    {
        self.median_blur(7)
    }
    pub fn median_blur9x9(&self) -> Result<Self>
    where
        Self: Sized,
    {
        self.median_blur(9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imgcodecs::Read;

    #[test]
    fn median_blur_tests() {
        let src = Mat::<u8, 1>::read("mock/lenna.png").unwrap();
        assert_eq!(src.cols(), 512);
        assert_eq!(src.rows(), 512);
        assert_eq!(src.channels(), 1);
        let dst = src.median_blur(5).unwrap();
        assert_eq!(src.cols(), dst.cols());
        assert_eq!(src.rows(), dst.rows());
        assert_eq!(src.channels(), dst.channels());
    }

    #[test]
    fn median_blur3x3_tests() {
        let src = Mat::<u8, 1>::read("mock/lenna.png").unwrap();
        assert_eq!(src.cols(), 512);
        assert_eq!(src.rows(), 512);
        assert_eq!(src.channels(), 1);
        let dst = src.median_blur3x3().unwrap();
        assert_eq!(src.cols(), dst.cols());
        assert_eq!(src.rows(), dst.rows());
        assert_eq!(src.channels(), dst.channels());
    }

    #[test]
    fn median_blur5x5_tests() {
        let src = Mat::<u8, 1>::read("mock/lenna.png").unwrap();
        assert_eq!(src.cols(), 512);
        assert_eq!(src.rows(), 512);
        assert_eq!(src.channels(), 1);
        let dst = src.median_blur5x5().unwrap();
        assert_eq!(src.cols(), dst.cols());
        assert_eq!(src.rows(), dst.rows());
        assert_eq!(src.channels(), dst.channels());
    }

    #[test]
    fn median_blur7x7_tests() {
        let src = Mat::<u8, 1>::read("mock/lenna.png").unwrap();
        assert_eq!(src.cols(), 512);
        assert_eq!(src.rows(), 512);
        assert_eq!(src.channels(), 1);
        let dst = src.median_blur7x7().unwrap();
        assert_eq!(src.cols(), dst.cols());
        assert_eq!(src.rows(), dst.rows());
        assert_eq!(src.channels(), dst.channels());
    }

    #[test]
    fn median_blur9x9_tests() {
        let src = Mat::<u8, 1>::read("mock/lenna.png").unwrap();
        assert_eq!(src.cols(), 512);
        assert_eq!(src.rows(), 512);
        assert_eq!(src.channels(), 1);
        let dst = src.median_blur9x9().unwrap();
        assert_eq!(src.cols(), dst.cols());
        assert_eq!(src.rows(), dst.rows());
        assert_eq!(src.channels(), dst.channels());
    }
}
