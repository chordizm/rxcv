#![allow(non_upper_case_globals)]
use crate::{core::Mat, imgproc::Connectivity, result::Result, Point, Rect};
use bitflags::bitflags;

bitflags! {
    pub struct FloodFillFlags: i32 {
        const FLOODFILL_FIXED_RANGE = 1 << 16;
        const FLOODFILL_MASK_ONLY = 1 << 17;
    }
}

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult, Point, Rect};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_flood_fill(
            src: *const MatPointer,
            mask: *const MatPointer,
            seed_point: Point,
            new_val: *const MatPointer,
            lo_diff: *const MatPointer,
            up_diff: *const MatPointer,
            flags: i32,
        ) -> FFIResult<Rect>;
    }
}

pub trait FloodFill<T, const C: usize>
where
    Self: Sized,
{
    fn flood_fill(
        &self,
        seed_point: Point,
        new_val: [T; C],
        lo_diff: [T; C],
        up_diff: [T; C],
        flags: FloodFillFlags,
        connectivity: Connectivity,
    ) -> Result<(Mat<u8, 1>, Rect)>;
}

macro_rules! impl_flood_fill_type {
    ($([$t:ty, $c:tt]),+) => {
        $(
            impl FloodFill<$t, $c> for Mat<$t, $c> {
                fn flood_fill(
                    &self,
                    seed_point: Point,
                    new_val: [$t; $c],
                    lo_diff: [$t; $c],
                    up_diff: [$t; $c],
                    flags: FloodFillFlags,
                    connectivity: Connectivity
                ) -> Result<(Mat<u8, 1>, Rect)> {
                    let dst = Mat::<u8, 1>::from_shape(self.rows() as usize + 2, self.cols() as usize + 2)?;
                    let new_val = Mat::<$t, $c>::from_shape_vec(1, $c, &new_val)?;
                    let lo_diff = Mat::<$t, $c>::from_shape_vec(1, $c, &lo_diff)?;
                    let up_diff = Mat::<$t, $c>::from_shape_vec(1, $c, &up_diff)?;
                    let rect = Result::from(unsafe {
                        ffi::cv_flood_fill(
                            self.pointer,
                            dst.pointer,
                            seed_point,
                            new_val.pointer,
                            lo_diff.pointer,
                            up_diff.pointer,
                            flags.bits() | connectivity.bits(),
                        )
                    })?;
                    Ok((dst, rect))
                }
            }
        )+
    };
}

macro_rules! impl_flood_fill {
    ($($c:tt),+) => {
        $(
            impl_flood_fill_type! {
                [u8, $c],
                [f32, $c],
                [f64, $c]
            }
        )+
    };
}

impl_flood_fill!(1, 3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flood_fill_test() {
        let src = Mat::mock_7x6_square_2x4_2x3().cvt_color_bgr2gray().unwrap();
        assert_eq!(src.cols(), 7);
        assert_eq!(src.rows(), 6);
        assert_eq!(src.channels(), 1);
        let seed_point = Point { x: 2, y: 2 };
        let new_val = [255];
        let lo_diff = [0];
        let up_diff = [0];
        let flags = FloodFillFlags::FLOODFILL_FIXED_RANGE | FloodFillFlags::FLOODFILL_MASK_ONLY;
        let (dst, _) = src
            .flood_fill(
                seed_point,
                new_val,
                lo_diff,
                up_diff,
                flags,
                Connectivity::NEIGHBORHOODS_4,
            )
            .unwrap();
        assert_eq!(dst.cols(), src.cols() + 2);
        assert_eq!(dst.rows(), src.rows() + 2);
        assert_eq!(dst.data()[20], 1);
        assert_eq!(dst.data()[19], 0);
        assert_eq!(src.data()[0], 0);
    }
}
