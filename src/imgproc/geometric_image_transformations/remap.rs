#![allow(non_upper_case_globals)]
use bitflags::bitflags;

use crate::{core::Mat, result::Result, BorderTypes};

bitflags! {
    pub struct RemapInterpolationFlags: i32 {
        const INTER_NEAREST = 0;
        const INTER_LINEAR = 1;
        const INTER_CUBIC = 2;
        const INTER_LANCZOS4 = 4;
        const INTER_NEAREST_EXACT = 6;
        const INTER_MAX = 7;
        const WARP_FILL_OUTLIERS = 8;
        const WARP_INVERSE_MAP = 16;
    }
}

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_remap(
            src: *const MatPointer,
            dst: *const MatPointer,
            map1: *const MatPointer,
            map2: *const MatPointer,
            interpolation: i32,
            border_mode: i32,
        ) -> FFIResult<i32>;
    }
}

pub trait Remap<T, Map, Interpolation, const C: usize>
where
    Self: Sized,
{
    fn remap(
        &self,
        map1: Mat<Map, C>,
        map2: Mat<Map, C>,
        interpolation: Interpolation,
        border_mode: BorderTypes,
    ) -> Result<Self>;
}

macro_rules! impl_remap {
    ($input_type:ty, $input_channel:tt, $map_type: ty, $interpolation: ty, $channel:tt) => {
        impl Remap<$input_type, $map_type, $interpolation, $channel>
            for Mat<$input_type, $input_channel>
        {
            fn remap(
                &self,
                map1: Mat<$map_type, $channel>,
                map2: Mat<$map_type, $channel>,
                interpolation: $interpolation,
                border_mode: BorderTypes,
            ) -> Result<Mat<$input_type, $input_channel>>
            where
                Self: Sized,
            {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_remap(
                        self.pointer,
                        dst.pointer,
                        map1.pointer,
                        map2.pointer,
                        interpolation.bits(),
                        border_mode.bits(),
                    )
                })?;
                Ok(dst)
            }
        }
    };
}
macro_rules! impl_remap_all {
    ($t:ty, $i: ty) => {
        impl_remap!($t, 1, f32, $i, 1);
        impl_remap!($t, 2, f32, $i, 1);
        impl_remap!($t, 3, f32, $i, 1);
    };
}

impl_remap_all!(u8, RemapInterpolationFlags);
impl_remap_all!(u16, RemapInterpolationFlags);
impl_remap_all!(i16, RemapInterpolationFlags);
impl_remap_all!(f32, RemapInterpolationFlags);
impl_remap_all!(f64, RemapInterpolationFlags);

#[cfg(test)]
mod tests {
    use super::{RemapInterpolationFlags, *};

    macro_rules! impl_remap_test {
        ($name:ident, $src_type:ty, $channels:tt) => {
            #[test]
            fn $name() {
                fn run_test(flag: RemapInterpolationFlags) {
                    let src = Mat::<$src_type, $channels>::from_shape(32, 32).unwrap();
                    let map1 = Mat::<f32, 1>::from_shape(16, 16).unwrap();
                    let map2 = Mat::<f32, 1>::from_shape(16, 16).unwrap();
                    let dst = src
                        .remap(map1, map2, flag, BorderTypes::BORDER_CONSTANT)
                        .unwrap();
                    assert_eq!(
                        dst.data_type().unwrap().bits(),
                        src.data_type().unwrap().bits()
                    );
                    assert_eq!(dst.channels(), $channels);
                    assert_eq!(dst.cols(), 16);
                    assert_eq!(dst.rows(), 16);
                }
                run_test(RemapInterpolationFlags::INTER_NEAREST);
                run_test(RemapInterpolationFlags::INTER_LINEAR);
                run_test(RemapInterpolationFlags::INTER_CUBIC);
                run_test(RemapInterpolationFlags::INTER_LANCZOS4)
            }
        };
    }
    impl_remap_test!(remap_8uc1_test, u8, 1);
    impl_remap_test!(remap_8uc2_test, u8, 2);
    impl_remap_test!(remap_8uc3_test, u8, 3);
    impl_remap_test!(remap_16uc1_test, u16, 1);
    impl_remap_test!(remap_16uc2_test, u16, 2);
    impl_remap_test!(remap_16uc3_test, u16, 3);
    impl_remap_test!(remap_16sc1_test, i16, 1);
    impl_remap_test!(remap_16sc2_test, i16, 2);
    impl_remap_test!(remap_16sc3_test, i16, 3);
    impl_remap_test!(remap_32fc1_test, f32, 1);
    impl_remap_test!(remap_32fc2_test, f32, 2);
    impl_remap_test!(remap_32fc3_test, f32, 3);
    impl_remap_test!(remap_64fc1_test, f64, 1);
    impl_remap_test!(remap_64fc2_test, f64, 2);
    impl_remap_test!(remap_64fc3_test, f64, 3);
}

mod i8 {
    #![allow(non_upper_case_globals)]
    use bitflags::bitflags;

    use super::{ffi, Remap};
    use crate::{core::Mat, result::Result, BorderTypes};

    bitflags! {
        pub struct RemapInterpolationFlags: i32 {
            const INTER_NEAREST = 0;
        }
    }

    impl_remap_all!(i8, RemapInterpolationFlags);

    #[cfg(test)]
    mod tests {
        use super::RemapInterpolationFlags;
        use crate::{imgproc::Remap, BorderTypes, Mat};
        macro_rules! impl_remap_test {
            ($name:ident, $c:tt, $interpolation: expr) => {
                #[test]
                fn $name() {
                    let src = Mat::<i8, $c>::from_shape(32, 32).unwrap();
                    let map1 = Mat::<f32, 1>::from_shape(16, 16).unwrap();
                    let map2 = Mat::<f32, 1>::from_shape(16, 16).unwrap();
                    let dst = src
                        .remap(map1, map2, $interpolation, BorderTypes::BORDER_CONSTANT)
                        .unwrap();
                    assert_eq!(
                        dst.data_type().unwrap().bits(),
                        src.data_type().unwrap().bits()
                    );
                    assert_eq!(dst.channels(), $c);
                    assert_eq!(dst.cols(), 16);
                    assert_eq!(dst.rows(), 16);
                }
            };
        }

        impl_remap_test!(
            remap_8sc1_inter_nearest_test,
            1,
            RemapInterpolationFlags::INTER_NEAREST
        );
        impl_remap_test!(
            remap_8sc2_inter_nearest_test,
            2,
            RemapInterpolationFlags::INTER_NEAREST
        );
        impl_remap_test!(
            remap_8sc3_inter_nearest_test,
            3,
            RemapInterpolationFlags::INTER_NEAREST
        );
    }
}

mod i32 {
    #![allow(non_upper_case_globals)]
    use bitflags::bitflags;

    use super::{ffi, Remap};
    use crate::{core::Mat, result::Result, BorderTypes};

    bitflags! {
        pub struct RemapInterpolationFlags: i32 {
            const INTER_NEAREST = 0;
        }
    }

    impl_remap_all!(i32, RemapInterpolationFlags);

    #[cfg(test)]
    mod tests {
        use super::RemapInterpolationFlags;
        use crate::{imgproc::Remap, BorderTypes, Mat};
        macro_rules! impl_remap_test {
            ($name:ident, $c:tt, $interpolation: expr) => {
                #[test]
                fn $name() {
                    let src = Mat::<i32, $c>::from_shape(32, 32).unwrap();
                    let map1 = Mat::<f32, 1>::from_shape(16, 16).unwrap();
                    let map2 = Mat::<f32, 1>::from_shape(16, 16).unwrap();
                    let dst = src
                        .remap(map1, map2, $interpolation, BorderTypes::BORDER_CONSTANT)
                        .unwrap();
                    assert_eq!(
                        dst.data_type().unwrap().bits(),
                        src.data_type().unwrap().bits()
                    );
                    assert_eq!(dst.channels(), $c);
                    assert_eq!(dst.cols(), 16);
                    assert_eq!(dst.rows(), 16);
                }
            };
        }

        impl_remap_test!(
            remap_32sc1_inter_nearest_test,
            1,
            RemapInterpolationFlags::INTER_NEAREST
        );
        impl_remap_test!(
            remap_32sc2_inter_nearest_test,
            2,
            RemapInterpolationFlags::INTER_NEAREST
        );
        impl_remap_test!(
            remap_32sc3_inter_nearest_test,
            3,
            RemapInterpolationFlags::INTER_NEAREST
        );
    }
}
