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

pub trait Remap<Interpolation, Map1, const C1: usize, Map2, const C2: usize>
where
    Self: Sized,
{
    fn remap(
        &self,
        map1: Mat<Map1, C1>,
        map2: Mat<Map2, C2>,
        interpolation: Interpolation,
        border_mode: BorderTypes,
    ) -> Result<Self>;
}

macro_rules! impl_remap {
    ($input_type:ty, $input_channel:tt, $interpolation: ty) => {
        impl<T, const C: usize> Remap<$interpolation, f32, 2, T, C>
            for Mat<$input_type, $input_channel>
        {
            fn remap(
                &self,
                map1: Mat<f32, 2>,
                _map2: Mat<T, C>,
                interpolation: $interpolation,
                border_mode: BorderTypes,
            ) -> Result<Mat<$input_type, $input_channel>>
            where
                Self: Sized,
            {
                let map2 = Mat::<T, C>::new()?;
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
        impl Remap<$interpolation, f32, 1, f32, 1> for Mat<$input_type, $input_channel> {
            fn remap(
                &self,
                map1: Mat<f32, 1>,
                map2: Mat<f32, 1>,
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

        impl Remap<$interpolation, i16, 2, u16, 1> for Mat<$input_type, $input_channel> {
            fn remap(
                &self,
                map1: Mat<i16, 2>,
                map2: Mat<u16, 1>,
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
        impl_remap!($t, 1, $i);
        impl_remap!($t, 2, $i);
        impl_remap!($t, 3, $i);
    };
}

impl_remap_all!(u8, RemapInterpolationFlags);
impl_remap_all!(u16, RemapInterpolationFlags);
impl_remap_all!(i16, RemapInterpolationFlags);
impl_remap_all!(f32, RemapInterpolationFlags);
impl_remap_all!(f64, RemapInterpolationFlags);

#[cfg(test)]
mod tests {
    macro_rules! impl_remap_test {
        ($name:ident, $src_type:ty, $channels:tt) => {
            mod $name {
                use super::super::*;
                fn run_test_32fc1(flag: RemapInterpolationFlags) {
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
                #[test]
                fn test_32fc1() {
                    run_test_32fc1(RemapInterpolationFlags::INTER_NEAREST);
                    run_test_32fc1(RemapInterpolationFlags::INTER_LINEAR);
                    run_test_32fc1(RemapInterpolationFlags::INTER_CUBIC);
                    run_test_32fc1(RemapInterpolationFlags::INTER_LANCZOS4);
                }

                fn run_test_32fc2(flag: RemapInterpolationFlags) {
                    let src = Mat::<$src_type, $channels>::from_shape(32, 32).unwrap();
                    let map1 = Mat::<f32, 2>::from_shape(16, 16).unwrap();
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

                #[test]
                fn test_32fc2() {
                    run_test_32fc2(RemapInterpolationFlags::INTER_NEAREST);
                    run_test_32fc2(RemapInterpolationFlags::INTER_LINEAR);
                    run_test_32fc2(RemapInterpolationFlags::INTER_CUBIC);
                    run_test_32fc2(RemapInterpolationFlags::INTER_LANCZOS4);
                }

                fn run_test_16sc2(flag: RemapInterpolationFlags) {
                    let src = Mat::<$src_type, $channels>::from_shape(32, 32).unwrap();
                    let map1 = Mat::<i16, 2>::from_shape(16, 16).unwrap();
                    let map2 = Mat::<u16, 1>::from_shape(16, 16).unwrap();
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

                #[test]
                fn test_16sc2() {
                    run_test_16sc2(RemapInterpolationFlags::INTER_NEAREST);
                    run_test_16sc2(RemapInterpolationFlags::INTER_LINEAR);
                    run_test_16sc2(RemapInterpolationFlags::INTER_CUBIC);
                    run_test_16sc2(RemapInterpolationFlags::INTER_LANCZOS4);
                }
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
        macro_rules! impl_remap_test {
            ($name:ident, $c:tt, $interpolation: expr) => {
                mod $name {
                    use super::super::*;
                    fn run_test_32fc1(flag: RemapInterpolationFlags) {
                        let src = Mat::<i8, $c>::from_shape(32, 32).unwrap();
                        let map1 = Mat::<f32, 1>::from_shape(16, 16).unwrap();
                        let map2 = Mat::<f32, 1>::from_shape(16, 16).unwrap();
                        let dst = src
                            .remap(map1, map2, flag, BorderTypes::BORDER_CONSTANT)
                            .unwrap();
                        assert_eq!(
                            dst.data_type().unwrap().bits(),
                            src.data_type().unwrap().bits()
                        );
                        assert_eq!(dst.channels(), $c);
                        assert_eq!(dst.cols(), 16);
                        assert_eq!(dst.rows(), 16);
                    }
                    #[test]
                    fn test_32fc1() {
                        run_test_32fc1(RemapInterpolationFlags::INTER_NEAREST);
                    }

                    fn run_test_32fc2(flag: RemapInterpolationFlags) {
                        let src = Mat::<i8, $c>::from_shape(32, 32).unwrap();
                        let map1 = Mat::<f32, 2>::from_shape(16, 16).unwrap();
                        let map2 = Mat::<f32, 1>::from_shape(16, 16).unwrap();
                        let dst = src
                            .remap(map1, map2, flag, BorderTypes::BORDER_CONSTANT)
                            .unwrap();
                        assert_eq!(
                            dst.data_type().unwrap().bits(),
                            src.data_type().unwrap().bits()
                        );
                        assert_eq!(dst.channels(), $c);
                        assert_eq!(dst.cols(), 16);
                        assert_eq!(dst.rows(), 16);
                    }

                    #[test]
                    fn test_32fc2() {
                        run_test_32fc2(RemapInterpolationFlags::INTER_NEAREST);
                    }

                    fn run_test_16sc2(flag: RemapInterpolationFlags) {
                        let src = Mat::<i8, $c>::from_shape(32, 32).unwrap();
                        let map1 = Mat::<i16, 2>::from_shape(16, 16).unwrap();
                        let map2 = Mat::<u16, 1>::from_shape(16, 16).unwrap();
                        let dst = src
                            .remap(map1, map2, flag, BorderTypes::BORDER_CONSTANT)
                            .unwrap();
                        assert_eq!(
                            dst.data_type().unwrap().bits(),
                            src.data_type().unwrap().bits()
                        );
                        assert_eq!(dst.channels(), $c);
                        assert_eq!(dst.cols(), 16);
                        assert_eq!(dst.rows(), 16);
                    }

                    #[test]
                    fn test_16sc2() {
                        run_test_16sc2(RemapInterpolationFlags::INTER_NEAREST);
                    }
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
        macro_rules! impl_remap_test {
            ($name:ident, $c:tt, $interpolation: expr) => {
                mod $name {
                    use super::super::*;
                    fn run_test_32fc1(flag: RemapInterpolationFlags) {
                        let src = Mat::<i32, $c>::from_shape(32, 32).unwrap();
                        let map1 = Mat::<f32, 1>::from_shape(16, 16).unwrap();
                        let map2 = Mat::<f32, 1>::from_shape(16, 16).unwrap();
                        let dst = src
                            .remap(map1, map2, flag, BorderTypes::BORDER_CONSTANT)
                            .unwrap();
                        assert_eq!(
                            dst.data_type().unwrap().bits(),
                            src.data_type().unwrap().bits()
                        );
                        assert_eq!(dst.channels(), $c);
                        assert_eq!(dst.cols(), 16);
                        assert_eq!(dst.rows(), 16);
                    }
                    #[test]
                    fn test_32fc1() {
                        run_test_32fc1(RemapInterpolationFlags::INTER_NEAREST);
                    }

                    fn run_test_32fc2(flag: RemapInterpolationFlags) {
                        let src = Mat::<i32, $c>::from_shape(32, 32).unwrap();
                        let map1 = Mat::<f32, 2>::from_shape(16, 16).unwrap();
                        let map2 = Mat::<f32, 1>::from_shape(16, 16).unwrap();
                        let dst = src
                            .remap(map1, map2, flag, BorderTypes::BORDER_CONSTANT)
                            .unwrap();
                        assert_eq!(
                            dst.data_type().unwrap().bits(),
                            src.data_type().unwrap().bits()
                        );
                        assert_eq!(dst.channels(), $c);
                        assert_eq!(dst.cols(), 16);
                        assert_eq!(dst.rows(), 16);
                    }

                    #[test]
                    fn test_32fc2() {
                        run_test_32fc2(RemapInterpolationFlags::INTER_NEAREST);
                    }

                    fn run_test_16sc2(flag: RemapInterpolationFlags) {
                        let src = Mat::<i32, $c>::from_shape(32, 32).unwrap();
                        let map1 = Mat::<i16, 2>::from_shape(16, 16).unwrap();
                        let map2 = Mat::<u16, 1>::from_shape(16, 16).unwrap();
                        let dst = src
                            .remap(map1, map2, flag, BorderTypes::BORDER_CONSTANT)
                            .unwrap();
                        assert_eq!(
                            dst.data_type().unwrap().bits(),
                            src.data_type().unwrap().bits()
                        );
                        assert_eq!(dst.channels(), $c);
                        assert_eq!(dst.cols(), 16);
                        assert_eq!(dst.rows(), 16);
                    }

                    #[test]
                    fn test_16sc2() {
                        run_test_16sc2(RemapInterpolationFlags::INTER_NEAREST);
                    }
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
