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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_type_test() {
        assert_eq!(make_type(DataTypes::CV_16U, 3), 18);
    }
}
