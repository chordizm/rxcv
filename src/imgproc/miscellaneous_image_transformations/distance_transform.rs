#![allow(non_upper_case_globals)]
use crate::{core::Mat, result::Result};
use bitflags::bitflags;

bitflags! {
    pub struct DistanceType: i32 {
        const DIST_USER = -1;
        const DIST_L1 = 1;
        const DIST_L2 = 2;
        const DIST_C = 3;
        const DIST_L12 = 4;
        const DIST_FAIR = 5;
        const DIST_WELSCH = 6;
        const DIST_HUBER = 7;
    }
}

bitflags! {
    pub struct DistanceTransformMasks: i32 {
        const DIST_MASK_3 = 3;
        const DIST_MASK_5 = 5;
    }
}

bitflags! {
    pub struct DistanceTransformLabelTypes: i32 {
        const DIST_LABEL_CCOMP = 0;
        const DIST_LABEL_PIXEL = 1;
    }
}

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_distance_transform(
            src: *const MatPointer,
            dst: *const MatPointer,
            labels: *const MatPointer,
            distance_type: i32,
            mask_size: i32,
            label_type: i32,
        ) -> FFIResult<i32>;
    }
}

pub trait DistanceTransform<T> {
    fn distance_transform(
        &self,
        distance_type: DistanceType,
        mask_size: DistanceTransformMasks,
        label_type: DistanceTransformLabelTypes,
    ) -> Result<(Mat<T, 1>, Mat<i32, 1>)>;
}

macro_rules! impl_distance_transform {
    ($t:ty) => {
        impl DistanceTransform<$t> for Mat<u8, 1> {
            fn distance_transform(
                &self,
                distance_type: DistanceType,
                mask_size: DistanceTransformMasks,
                label_type: DistanceTransformLabelTypes,
            ) -> Result<(Mat<$t, 1>, Mat<i32, 1>)> {
                let dst = Mat::<$t, 1>::new()?;
                let labels = Mat::<i32, 1>::new()?;
                Result::from(unsafe {
                    ffi::cv_distance_transform(
                        self.pointer,
                        dst.pointer,
                        labels.pointer,
                        distance_type.bits(),
                        mask_size.bits(),
                        label_type.bits(),
                    )
                })?;
                Ok((dst, labels))
            }
        }
    };
}

impl_distance_transform!(u8);
impl_distance_transform!(f32);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! distance_transform_test {
        ($name: ident, $t:ty) => {
            #[test]
            fn $name() {
                let src = Mat::<u8, 1>::from_shape(32, 32).unwrap();
                let (dst, labels): (Mat<$t, 1>, Mat<i32, 1>) = src
                    .distance_transform(
                        DistanceType::DIST_L1,
                        DistanceTransformMasks::DIST_MASK_3,
                        DistanceTransformLabelTypes::DIST_LABEL_CCOMP,
                    )
                    .unwrap();
                assert_eq!(src.cols(), dst.cols());
                assert_eq!(src.rows(), dst.rows());
                assert_eq!(1, dst.channels());
                assert_eq!(src.cols(), labels.cols());
                assert_eq!(src.rows(), labels.rows());
                assert_eq!(1, labels.channels());
            }
        };
    }

    distance_transform_test!(distance_transform_8uc1_test, u8);
    distance_transform_test!(distance_transform_32fcc1_test, f32);
}
