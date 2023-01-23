use super::consts::{ContourApproximationModes, RetrievalModes};
use crate::core::{Contours, Mat};

mod ffi {
    use crate::{
        core::{ContoursPointer, MatPointer},
        ffi::FFIResult,
    };

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_find_contours(
            src: *const MatPointer,
            contours: *const ContoursPointer,
            mode: i32,
            method: i32,
        ) -> FFIResult<i32>;
    }
}

pub trait FindContours {
    fn find_contours(
        &self,
        mode: RetrievalModes,
        method: ContourApproximationModes,
    ) -> Result<Contours, &'static str>;
}

impl<T> FindContours for Mat<T, 1> {
    fn find_contours(
        &self,
        mode: RetrievalModes,
        method: ContourApproximationModes,
    ) -> Result<Contours, &'static str>
    where
        Self: Sized,
    {
        let mut contours = Contours::default();
        Result::from(unsafe {
            ffi::cv_find_contours(self.pointer, contours.pointer, mode.bits(), method.bits())
        })?;
        contours.inner = contours.get_inner();
        Ok(contours)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_contours_test() {
        let src = Mat::mock_7x6_square_5x4().cvt_color_bgr2gray().unwrap();
        let contours = src
            .find_contours(
                RetrievalModes::RETR_EXTERNAL,
                ContourApproximationModes::CHAIN_APPROX_NONE,
            )
            .unwrap();
        assert_eq!(contours.size(), 1);

        let contour = &contours[0];
        assert_eq!(contour.size(), 14);
        assert_eq!(contour.area(), 4. * 3.);
        assert_eq!(contour.closed_arc_length(), 4. * 2. + 3. * 2.);
        assert_eq!(contour.arc_length(false), 4. * 2. + 3. * 2. - 1.);
    }

    #[test]
    fn find_2contours_test() {
        let src = Mat::mock_7x6_square_2x4_2x3().cvt_color_bgr2gray().unwrap();
        let contours = src
            .find_contours(
                RetrievalModes::RETR_EXTERNAL,
                ContourApproximationModes::CHAIN_APPROX_NONE,
            )
            .unwrap();
        assert_eq!(contours.size(), 2);
    }
}
