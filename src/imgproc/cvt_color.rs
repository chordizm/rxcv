use super::consts::ColorConversionCodes;
use crate::core::Mat;

mod ffi {
    use crate::core::MatPointer;

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_cvt_color(
            src: *const MatPointer,
            dst: *const MatPointer,
            code: i32,
        ) -> bool;
    }
}

pub trait CvtColor {
    fn cvt_color(&self, code: ColorConversionCodes) -> Result<Self, &'static str>
    where
        Self: Sized;
}

impl<T> Mat<T, 3> {
    pub fn cvt_color_bgr2gray(&self) -> Result<Mat<T, 1>, &'static str> {
        let dst = Mat::default();
        if unsafe {
            ffi::cv_cvt_color(
                self.pointer,
                dst.pointer,
                ColorConversionCodes::COLOR_BGR2GRAY.bits(),
            )
        } {
            Ok(dst)
        } else {
            Err("Failed to operation.")
        }
    }

    pub fn cvt_color_bgr2hsv(&self) -> Result<Self, &'static str> {
        let dst = Mat::default();
        if unsafe {
            ffi::cv_cvt_color(
                self.pointer,
                dst.pointer,
                ColorConversionCodes::COLOR_BGR2HSV.bits(),
            )
        } {
            Ok(dst)
        } else {
            Err("Failed to operation.")
        }
    }
}

impl<T> Mat<T, 1> {
    pub fn cvt_color_gray2bgr(&self) -> Result<Mat<T, 3>, &'static str> {
        let dst = Mat::default();
        if unsafe {
            ffi::cv_cvt_color(
                self.pointer,
                dst.pointer,
                ColorConversionCodes::COLOR_GRAY2BGR.bits(),
            )
        } {
            Ok(dst)
        } else {
            Err("Failed to operation.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cvt_color_test() {
        let src = Mat::mock_7x6_white();
        assert_eq!(src.channels(), 3);
        let gray = src.cvt_color_bgr2gray().unwrap();
        assert_eq!(src.channels(), 3);
        assert_eq!(gray.channels(), 1);
        let bgr = gray.cvt_color_gray2bgr().unwrap();
        assert_eq!(src.channels(), 3);
        assert_eq!(gray.channels(), 1);
        assert_eq!(bgr.channels(), 3);
        let hsv = src.cvt_color_bgr2hsv().unwrap();
        assert_eq!(src.channels(), 3);
        assert_eq!(hsv.channels(), 3);
    }
}
