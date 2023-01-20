use crate::core::Mat;

mod ffi {
    use crate::core::MatPointer;

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_median_blur(
            src: *const MatPointer,
            dst: *const MatPointer,
            ksize: i32,
        ) -> bool;
    }
}

pub trait MedianBlur {
    fn median_blur(&self, ksize: i32) -> Result<Self, &'static str>
    where
        Self: Sized;
    fn median_blur3x3(&self) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        self.median_blur(3)
    }
    fn median_blur5x5(&self) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        self.median_blur(5)
    }
    fn median_blur7x7(&self) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        self.median_blur(7)
    }
    fn median_blur9x9(&self) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        self.median_blur(9)
    }
}

impl<T, const C: usize> MedianBlur for Mat<T, C> {
    fn median_blur(&self, ksize: i32) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let dst = Mat::default();
        if unsafe { ffi::cv_median_blur(self.pointer, dst.pointer, ksize) } {
            Ok(dst)
        } else {
            Err("Failed to Operation")
        }
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
