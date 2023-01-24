use bitflags::bitflags;

bitflags! {
    pub struct DataTypes: i32 {
        const CV_8U = 0;
        const CV_8S = 1;
        const CV_16U = 2;
        const CV_16S = 3;
        const CV_32S = 4;
        const CV_32F = 5;
        const CV_64F = 6;
        const CV_16F = 7;
        const CV_8UC1 = make_type(Self::CV_8U, 1);
        const CV_8UC2 = make_type(Self::CV_8U, 2);
        const CV_8UC3 = make_type(Self::CV_8U, 3);
        const CV_8UC4 = make_type(Self::CV_8U, 4);
        const CV_8SC1 = make_type(Self::CV_8S, 1);
        const CV_8SC2 = make_type(Self::CV_8S, 2);
        const CV_8SC3 = make_type(Self::CV_8S, 3);
        const CV_8SC4 = make_type(Self::CV_8S, 4);
        const CV_16UC1 = make_type(Self::CV_16U, 1);
        const CV_16UC2 = make_type(Self::CV_16U, 2);
        const CV_16UC3 = make_type(Self::CV_16U, 3);
        const CV_16UC4 = make_type(Self::CV_16U, 4);
        const CV_16SC1 = make_type(Self::CV_16S, 1);
        const CV_16SC2 = make_type(Self::CV_16S, 2);
        const CV_16SC3 = make_type(Self::CV_16S, 3);
        const CV_16SC4 = make_type(Self::CV_16S, 4);
        const CV_32SC1 = make_type(Self::CV_32S, 1);
        const CV_32SC2 = make_type(Self::CV_32S, 2);
        const CV_32SC3 = make_type(Self::CV_32S, 3);
        const CV_32SC4 = make_type(Self::CV_32S, 4);
        const CV_32FC1 = make_type(Self::CV_32F, 1);
        const CV_32FC2 = make_type(Self::CV_32F, 2);
        const CV_32FC3 = make_type(Self::CV_32F, 3);
        const CV_32FC4 = make_type(Self::CV_32F, 4);
        const CV_64FC1 = make_type(Self::CV_64F, 1);
        const CV_64FC2 = make_type(Self::CV_64F, 2);
        const CV_64FC3 = make_type(Self::CV_64F, 3);
        const CV_64FC4 = make_type(Self::CV_64F, 4);
        const CV_16FC1 = make_type(Self::CV_16F, 1);
        const CV_16FC2 = make_type(Self::CV_16F, 2);
        const CV_16FC3 = make_type(Self::CV_16F, 3);
        const CV_16FC4 = make_type(Self::CV_16F, 4);
    }
}

pub(super) const fn make_type(depth: DataTypes, ch: i32) -> i32 {
    let channel_shift = 3;
    let depth_max = 1 << channel_shift;
    let mat_depth_mask = depth_max - 1;
    let mat_depth = depth.bits & mat_depth_mask;
    mat_depth + ((ch - 1) << channel_shift)
}

bitflags! {
    pub struct BorderTypes: i32 {
        const BORDER_CONSTANT = 0;
        const BORDER_REPLICATE = 1;
        const BORDER_REFLECT = 2;
        const BORDER_WRAP = 3;
        const BORDER_REFLECT_101 = 4;
        const BORDER_TRANSPARENT = 5;
        const BORDER_REFLECT101 = 4;
        const BORDER_DEFAULT = 4;
        const BORDER_ISOLATED = 16;
    }
}

#[repr(C)]
pub struct PointBase<T> {
    pub x: T,
    pub y: T,
}

pub type Point2i = PointBase<i32>;
pub type Point2f = PointBase<f32>;
pub type Point = Point2i;

impl Default for Point {
    fn default() -> Self {
        Self { x: -1, y: -1 }
    }
}

#[repr(C)]
pub struct SizeT<T> {
    pub width: T,
    pub height: T,
}

pub type Size2i = SizeT<i32>;
pub type Size = Size2i;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_type_test() {
        assert_eq!(make_type(DataTypes::CV_16U, 3), 18);
    }

    #[test]
    fn data_type_test() {
        assert_eq!(DataTypes::CV_8UC1.bits(), 0);
        assert_eq!(DataTypes::CV_8UC2.bits(), 8);
        assert_eq!(DataTypes::CV_8UC3.bits(), 16);
        assert_eq!(DataTypes::CV_8SC1.bits(), 1);
        assert_eq!(DataTypes::CV_8SC2.bits(), 9);
        assert_eq!(DataTypes::CV_8SC3.bits(), 17);
        assert_eq!(DataTypes::CV_16UC1.bits(), 2);
        assert_eq!(DataTypes::CV_16UC2.bits(), 10);
        assert_eq!(DataTypes::CV_16UC3.bits(), 18);
        assert_eq!(DataTypes::CV_16SC1.bits(), 3);
        assert_eq!(DataTypes::CV_16SC2.bits(), 11);
        assert_eq!(DataTypes::CV_16SC3.bits(), 19);
        assert_eq!(DataTypes::CV_32SC1.bits(), 4);
        assert_eq!(DataTypes::CV_32SC2.bits(), 12);
        assert_eq!(DataTypes::CV_32SC3.bits(), 20);
        assert_eq!(DataTypes::CV_32FC1.bits(), 5);
        assert_eq!(DataTypes::CV_32FC2.bits(), 13);
        assert_eq!(DataTypes::CV_32FC3.bits(), 21);
        assert_eq!(DataTypes::CV_64FC1.bits(), 6);
        assert_eq!(DataTypes::CV_64FC2.bits(), 14);
        assert_eq!(DataTypes::CV_64FC3.bits(), 22);
    }
}
