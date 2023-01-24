use crate::{
    core::Mat,
    imgproc::{InterpolationFlags, WarpPolarMode},
    result::Result,
    Point2f, Size,
};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult, Point2f, Size};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_warp_polar(
            src: *const MatPointer,
            dst: *const MatPointer,
            dsize: Size,
            center: Point2f,
            max_radius: f64,
            flags: i32,
        ) -> FFIResult<i32>;
    }
}

pub trait WarpPolar
where
    Self: Sized,
{
    fn warp_polar(
        &self,
        dsize: Size,
        center: Point2f,
        max_radius: f64,
        flags: InterpolationFlags,
        mode: WarpPolarMode,
    ) -> Result<Self>;
}

macro_rules! impl_wrap_polar {
    ($t:ty, $c:tt) => {
        impl WarpPolar for Mat<$t, $c> {
            fn warp_polar(
                &self,
                dsize: Size,
                center: Point2f,
                max_radius: f64,
                flags: InterpolationFlags,
                mode: WarpPolarMode,
            ) -> Result<Self>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_warp_polar(
                        self.pointer,
                        dst.pointer,
                        dsize,
                        center,
                        max_radius,
                        flags.bits() | mode.bits(),
                    )
                })?;
                Ok(dst)
            }
        }
    };
}

impl_wrap_polar!(f32, 1);
impl_wrap_polar!(f64, 1);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! warp_polar_test {
        ($name:ident, $t: ty, $c: tt) => {
            #[test]
            fn $name() {
                let src = Mat::<$t, $c>::from_shape(32, 32).unwrap();
                let dsize = Size {
                    width: 32,
                    height: 32,
                };
                let center = Point2f { x: 0., y: 0. };
                let dst = src
                    .warp_polar(
                        dsize,
                        center,
                        0.,
                        InterpolationFlags::INTER_NEAREST,
                        WarpPolarMode::WARP_POLAR_LOG,
                    )
                    .unwrap();
                assert_eq!(dst.channels(), $c);
                assert_eq!(dst.cols(), 32);
                assert_eq!(dst.rows(), 32);
            }
        };
    }
    warp_polar_test!(warp_polar_32fc1_test, f32, 1);
    warp_polar_test!(warp_polar_64fc1_test, f64, 1);
}
