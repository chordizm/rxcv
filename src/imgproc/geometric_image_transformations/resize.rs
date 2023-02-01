#![allow(non_upper_case_globals)]
use crate::{core::Mat, result::Result, Size};
use bitflags::bitflags;

bitflags! {
    pub struct ResizeInterpolationFlags: i32 {
        const INTER_NEAREST = 0;
        const INTER_LINEAR = 1;
        const INTER_CUBIC = 2;
        const INTER_AREA = 3;
        const INTER_LANCZOS4 = 4;
        const INTER_LINEAR_EXACT = 5;
        const INTER_NEAREST_EXACT = 6;
    }
}

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult, Size};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_resize(
            src: *const MatPointer,
            dst: *const MatPointer,
            size: Size,
            fx: f64,
            fy: f64,
            interpolation: i32,
        ) -> FFIResult<i32>;
    }
}

pub trait Resize<I>
where
    Self: Sized,
{
    fn resize(&self, size: Size, interpolation: I) -> Result<Self>;
    fn resize_factor(&self, fx: f64, fy: f64, interpolation: I) -> Result<Self>;
}

macro_rules! impl_resize {
    ($t: ty, $c: tt) => {
        impl Resize<ResizeInterpolationFlags> for Mat<$t, $c> {
            fn resize(&self, size: Size, interpolation: ResizeInterpolationFlags) -> Result<Self> {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_resize(
                        self.pointer,
                        dst.pointer,
                        size,
                        0.,
                        0.,
                        interpolation.bits(),
                    )
                })?;
                Ok(dst)
            }
            fn resize_factor(
                &self,
                fx: f64,
                fy: f64,
                interpolation: ResizeInterpolationFlags,
            ) -> Result<Self> {
                let dst = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_resize(
                        self.pointer,
                        dst.pointer,
                        Size::default(),
                        fx,
                        fy,
                        interpolation.bits(),
                    )
                })?;
                Ok(dst)
            }
        }
    };
}

macro_rules! impl_resize_all {
    ($t: ty) => {
        impl_resize!($t, 1);
        impl_resize!($t, 2);
        impl_resize!($t, 3);
    };
}

impl_resize_all!(u8);
impl_resize_all!(u16);
impl_resize_all!(i16);
impl_resize_all!(f32);
impl_resize_all!(f64);

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! resize_test {
        ($name:ident, $t: ty) => {
            #[test]
            fn $name() {
                fn run_test(interpolation: ResizeInterpolationFlags) {
                    let src = Mat::<$t, 3>::from_shape(32, 32).unwrap();
                    let dst1 = src
                        .resize(
                            Size {
                                width: 16,
                                height: 16,
                            },
                            interpolation,
                        )
                        .unwrap();
                    let dst2 = src.resize_factor(0.5, 0.5, interpolation).unwrap();
                    assert_eq!(dst1.cols(), 16);
                    assert_eq!(dst1.rows(), 16);
                    assert_eq!(dst2.cols(), 16);
                    assert_eq!(dst2.rows(), 16);
                }
                run_test(ResizeInterpolationFlags::INTER_NEAREST);
                run_test(ResizeInterpolationFlags::INTER_LINEAR);
                run_test(ResizeInterpolationFlags::INTER_CUBIC);
                run_test(ResizeInterpolationFlags::INTER_AREA);
                run_test(ResizeInterpolationFlags::INTER_LANCZOS4);
                run_test(ResizeInterpolationFlags::INTER_LINEAR_EXACT);
                run_test(ResizeInterpolationFlags::INTER_NEAREST_EXACT);
            }
        };
    }
    resize_test!(resize_u8_test, u8);
    resize_test!(resize_u16_test, u16);
    resize_test!(resize_i16_test, i16);
    resize_test!(resize_f32_test, f32);
    resize_test!(resize_f64_test, f64);
}

macro_rules! impl_module {
    ($name: ident, $t: ty) => {
        mod $name {
            #![allow(non_upper_case_globals)]
            use crate::{core::Mat, result::Result, Size};
            use bitflags::bitflags;

            bitflags! {
                pub struct ResizeInterpolationFlags: i32 {
                    const INTER_NEAREST = 0;
                }
            }

            macro_rules! impl_resize {
                ($c: tt) => {
                    impl super::Resize<ResizeInterpolationFlags> for Mat<$t, $c> {
                        fn resize(
                            &self,
                            size: Size,
                            interpolation: ResizeInterpolationFlags,
                        ) -> Result<Self> {
                            let dst = Mat::new()?;
                            Result::from(unsafe {
                                super::ffi::cv_resize(
                                    self.pointer,
                                    dst.pointer,
                                    size,
                                    0.,
                                    0.,
                                    interpolation.bits(),
                                )
                            })?;
                            Ok(dst)
                        }
                        fn resize_factor(
                            &self,
                            fx: f64,
                            fy: f64,
                            interpolation: ResizeInterpolationFlags,
                        ) -> Result<Self> {
                            let dst = Mat::new()?;
                            Result::from(unsafe {
                                super::ffi::cv_resize(
                                    self.pointer,
                                    dst.pointer,
                                    Size::default(),
                                    fx,
                                    fy,
                                    interpolation.bits(),
                                )
                            })?;
                            Ok(dst)
                        }
                    }
                };
            }

            impl_resize!(1);
            impl_resize!(2);
            impl_resize!(3);

            #[cfg(test)]
            mod tests {
                use super::super::Resize;
                use super::*;
                #[test]
                fn resize_test() {
                    fn run_test(interpolation: ResizeInterpolationFlags) {
                        let src = Mat::<$t, 3>::from_shape(32, 32).unwrap();
                        let dst1 = src
                            .resize(
                                Size {
                                    width: 16,
                                    height: 16,
                                },
                                interpolation,
                            )
                            .unwrap();
                        let dst2 = src.resize_factor(0.5, 0.5, interpolation).unwrap();
                        assert_eq!(dst1.cols(), 16);
                        assert_eq!(dst1.rows(), 16);
                        assert_eq!(dst2.cols(), 16);
                        assert_eq!(dst2.rows(), 16);
                    }
                    run_test(ResizeInterpolationFlags::INTER_NEAREST);
                }
            }
        }
    };
}

impl_module!(i8, i8);
impl_module!(i32, i32);
