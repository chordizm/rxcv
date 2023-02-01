use crate::{core::Mat, result::Result, DataTypes};

mod ffi {
    use crate::{core::MatPointer, ffi::FFIResult};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_convert_maps(
            map1: *const MatPointer,
            map2: *const MatPointer,
            dst1: *const MatPointer,
            dst2: *const MatPointer,
            dstmap1type: i32,
            nninterpolation: bool,
        ) -> FFIResult<i32>;
    }
}

pub trait ConvertMaps {
    fn convert_maps_16sc2(&self) -> Result<(Mat<i16, 2>, Mat<u16, 1>)>;
    fn convert_maps_32fc1(&self) -> Result<(Mat<f32, 1>, Mat<f32, 1>)>;
    fn convert_maps_32fc2(&self) -> Result<Mat<f32, 2>>;
}

impl ConvertMaps for Mat<f32, 2> {
    fn convert_maps_16sc2(&self) -> Result<(Mat<i16, 2>, Mat<u16, 1>)> {
        let _map2 = Mat::<u8, 1>::new()?;
        let dst1 = Mat::new()?;
        let dst2 = Mat::new()?;
        Result::from(unsafe {
            ffi::cv_convert_maps(
                self.pointer,
                _map2.pointer,
                dst1.pointer,
                dst2.pointer,
                DataTypes::CV_16SC2.bits(),
                false,
            )
        })?;
        Ok((dst1, dst2))
    }

    fn convert_maps_32fc1(&self) -> Result<(Mat<f32, 1>, Mat<f32, 1>)> {
        let _map2 = Mat::<u8, 1>::new()?;
        let dst1 = Mat::new()?;
        let dst2 = Mat::new()?;
        Result::from(unsafe {
            ffi::cv_convert_maps(
                self.pointer,
                _map2.pointer,
                dst1.pointer,
                dst2.pointer,
                DataTypes::CV_32FC1.bits(),
                false,
            )
        })?;
        Ok((dst1, dst2))
    }

    fn convert_maps_32fc2(&self) -> Result<Mat<f32, 2>> {
        let _map2 = Mat::<u8, 1>::new()?;
        let dst1 = Mat::new()?;
        let _dst2 = Mat::<u8, 1>::new()?;
        Result::from(unsafe {
            ffi::cv_convert_maps(
                self.pointer,
                _map2.pointer,
                dst1.pointer,
                _dst2.pointer,
                DataTypes::CV_32FC2.bits(),
                false,
            )
        })?;
        Ok(dst1)
    }
}

pub trait ConvertMapsWithInterpolation {
    fn convert_maps_16sc2_with_interpolation(&self) -> Result<Mat<i16, 2>>;
    fn convert_maps_32fc1_with_interpolation(&self) -> Result<Mat<f32, 1>>;
    fn convert_maps_32fc2_with_interpolation(&self) -> Result<Mat<f32, 2>>;
}

impl ConvertMapsWithInterpolation for Mat<f32, 2> {
    fn convert_maps_16sc2_with_interpolation(&self) -> Result<Mat<i16, 2>> {
        let _map2 = Mat::<u8, 1>::new()?;
        let dst1 = Mat::new()?;
        let _dst2 = Mat::<u8, 1>::new()?;
        Result::from(unsafe {
            ffi::cv_convert_maps(
                self.pointer,
                _map2.pointer,
                dst1.pointer,
                _dst2.pointer,
                DataTypes::CV_16SC2.bits(),
                true,
            )
        })?;
        Ok(dst1)
    }

    fn convert_maps_32fc1_with_interpolation(&self) -> Result<Mat<f32, 1>> {
        let _map2 = Mat::<u8, 1>::new()?;
        let dst1 = Mat::new()?;
        let _dst2 = Mat::<u8, 1>::new()?;
        Result::from(unsafe {
            ffi::cv_convert_maps(
                self.pointer,
                _map2.pointer,
                dst1.pointer,
                _dst2.pointer,
                DataTypes::CV_32FC1.bits(),
                true,
            )
        })?;
        Ok(dst1)
    }

    fn convert_maps_32fc2_with_interpolation(&self) -> Result<Mat<f32, 2>> {
        let _map2 = Mat::<u8, 1>::new()?;
        let dst1 = Mat::new()?;
        let _dst2 = Mat::<u8, 1>::new()?;
        Result::from(unsafe {
            ffi::cv_convert_maps(
                self.pointer,
                _map2.pointer,
                dst1.pointer,
                _dst2.pointer,
                DataTypes::CV_32FC2.bits(),
                true,
            )
        })?;
        Ok(dst1)
    }
}

pub trait ConvertMaps32FC2<M2> {
    fn convert_maps_32fc2(&self, map2: &M2) -> Result<Mat<f32, 2>>;
}

pub trait ConvertMapsBase<M2, D1, D2> {
    fn convert_maps(&self, map2: &M2) -> Result<(D1, D2)>;
}

macro_rules! impl_convert_maps {
    ([$map1_t:ty, $map1_c:tt], [$map2_t:ty, $map2_c:tt],[$dst_t:ty, $dst_c:tt], $code:expr) => {
        impl ConvertMaps32FC2<Mat<$map2_t, $map2_c>> for Mat<$map1_t, $map1_c> {
            fn convert_maps_32fc2(&self, map2: &Mat<$map2_t, $map2_c>) -> Result<Mat<f32, 2>> {
                let dst1 = Mat::new()?;
                let _dst2 = Mat::<u8, 1>::new()?;
                Result::from(unsafe {
                    ffi::cv_convert_maps(
                        self.pointer,
                        map2.pointer,
                        dst1.pointer,
                        _dst2.pointer,
                        $code,
                        false,
                    )
                })?;
                Ok(dst1)
            }
        }
    };
    ([$map1_t:ty, $map1_c:tt], [$map2_t:ty, $map2_c:tt],[$dst1_t:ty, $dst1_c:tt],[$dst2_t:ty, $dst2_c:tt], $code:expr) => {
        impl ConvertMapsBase<Mat<$map2_t, $map2_c>, Mat<$dst1_t, $dst1_c>, Mat<$dst2_t, $dst2_c>>
            for Mat<$map1_t, $map1_c>
        {
            fn convert_maps(
                &self,
                map2: &Mat<$map2_t, $map2_c>,
            ) -> Result<(Mat<$dst1_t, $dst1_c>, Mat<$dst2_t, $dst2_c>)> {
                let dst1 = Mat::new()?;
                let dst2 = Mat::new()?;
                Result::from(unsafe {
                    ffi::cv_convert_maps(
                        self.pointer,
                        map2.pointer,
                        dst1.pointer,
                        dst2.pointer,
                        $code,
                        false,
                    )
                })?;
                Ok((dst1, dst2))
            }
        }
    };
}

impl_convert_maps!(
    [i16, 2],
    [i16, 1],
    [i16, 2],
    [u16, 1],
    DataTypes::CV_16SC2.bits()
);
impl_convert_maps!(
    [i16, 2],
    [i16, 1],
    [f32, 1],
    [f32, 1],
    DataTypes::CV_32FC1.bits()
);
impl_convert_maps!([i16, 2], [i16, 1], [f32, 1], DataTypes::CV_32FC2.bits());

impl_convert_maps!(
    [i16, 2],
    [u16, 1],
    [i16, 2],
    [u16, 1],
    DataTypes::CV_16SC2.bits()
);
impl_convert_maps!(
    [i16, 2],
    [u16, 1],
    [f32, 1],
    [f32, 1],
    DataTypes::CV_32FC1.bits()
);
impl_convert_maps!([i16, 2], [u16, 1], [f32, 1], DataTypes::CV_32FC2.bits());

impl_convert_maps!(
    [i16, 1],
    [i16, 2],
    [i16, 2],
    [u16, 1],
    DataTypes::CV_16SC2.bits()
);
impl_convert_maps!(
    [i16, 1],
    [i16, 2],
    [f32, 1],
    [f32, 1],
    DataTypes::CV_32FC1.bits()
);
impl_convert_maps!([i16, 1], [i16, 2], [f32, 1], DataTypes::CV_32FC2.bits());

impl_convert_maps!(
    [u16, 1],
    [i16, 2],
    [i16, 2],
    [u16, 1],
    DataTypes::CV_16SC2.bits()
);
impl_convert_maps!(
    [u16, 1],
    [i16, 2],
    [f32, 1],
    [f32, 1],
    DataTypes::CV_32FC1.bits()
);
impl_convert_maps!([u16, 1], [i16, 2], [f32, 1], DataTypes::CV_32FC2.bits());

impl_convert_maps!(
    [f32, 1],
    [f32, 1],
    [i16, 2],
    [u16, 1],
    DataTypes::CV_16SC2.bits()
);
impl_convert_maps!(
    [f32, 1],
    [f32, 1],
    [f32, 1],
    [f32, 1],
    DataTypes::CV_32FC1.bits()
);
impl_convert_maps!([f32, 1], [f32, 1], [f32, 1], DataTypes::CV_32FC2.bits());

pub trait ConvertMapsBaseWithInterpolation<M2, D1> {
    fn convert_maps_with_interpolation(&self, map2: &M2) -> Result<D1>;
}

macro_rules! impl_convert_maps_with_interpolation {
    ([$map1_t:ty, $map1_c:tt], [$map2_t:ty, $map2_c:tt],[$dst_t:ty, $dst_c:tt], $code:expr) => {
        impl ConvertMapsBaseWithInterpolation<Mat<$map2_t, $map2_c>, Mat<$dst_t, $dst_c>>
            for Mat<$map1_t, $map1_c>
        {
            fn convert_maps_with_interpolation(
                &self,
                map2: &Mat<$map2_t, $map2_c>,
            ) -> Result<Mat<$dst_t, $dst_c>> {
                let dst1 = Mat::new()?;
                let dst2 = Mat::<$dst_t, $dst_c>::new()?;
                Result::from(unsafe {
                    ffi::cv_convert_maps(
                        self.pointer,
                        map2.pointer,
                        dst1.pointer,
                        dst2.pointer,
                        $code,
                        false,
                    )
                })?;
                Ok(dst1)
            }
        }
    };
}

impl_convert_maps_with_interpolation!([i16, 2], [i16, 1], [i16, 2], DataTypes::CV_16SC2.bits());
impl_convert_maps_with_interpolation!([i16, 2], [i16, 1], [f32, 2], DataTypes::CV_32FC2.bits());
impl_convert_maps_with_interpolation!([i16, 2], [u16, 1], [i16, 2], DataTypes::CV_16SC2.bits());
impl_convert_maps_with_interpolation!([i16, 2], [u16, 1], [f32, 2], DataTypes::CV_32FC2.bits());
impl_convert_maps_with_interpolation!([i16, 1], [i16, 2], [i16, 2], DataTypes::CV_16SC2.bits());
impl_convert_maps_with_interpolation!([i16, 1], [i16, 2], [f32, 2], DataTypes::CV_32FC2.bits());
impl_convert_maps_with_interpolation!([u16, 1], [i16, 2], [i16, 2], DataTypes::CV_16SC2.bits());
impl_convert_maps_with_interpolation!([u16, 1], [i16, 2], [f32, 2], DataTypes::CV_32FC2.bits());
impl_convert_maps_with_interpolation!([f32, 1], [f32, 1], [i16, 2], DataTypes::CV_16SC2.bits());
impl_convert_maps_with_interpolation!([f32, 1], [f32, 1], [f32, 1], DataTypes::CV_32FC1.bits());
impl_convert_maps_with_interpolation!([f32, 1], [f32, 1], [f32, 2], DataTypes::CV_32FC2.bits());

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! impl_convert_maps_test {
        ($name:ident, [$map1_t:ty, $map1_c:tt], [$map2_t:ty, $map2_c:tt],[$dst_t:ty, $dst_c:tt], $code:expr) => {
            #[test]
            fn $name() {
                let map1 = Mat::<$map1_t, $map1_c>::from_shape(32, 32).unwrap();
                let map2 = Mat::<$map2_t, $map2_c>::from_shape(32, 32).unwrap();
                let dst: Mat<$dst_t, $dst_c> = map1.convert_maps_32fc2(&map2).unwrap();
                assert_eq!(dst.data_type().unwrap().bits(), $code);
                assert_eq!(dst.cols(), map1.cols());
                assert_eq!(dst.rows(), map1.rows());
            }
        };
        ($name:ident, [$map1_t:ty, $map1_c:tt], [$map2_t:ty, $map2_c:tt],[$dst1_t:ty, $dst1_c:tt],[$dst2_t:ty, $dst2_c:tt], $code1:expr, $code2:expr) => {
            #[test]
            fn $name() {
                let map1 = Mat::<$map1_t, $map1_c>::from_shape(32, 32).unwrap();
                let map2 = Mat::<$map2_t, $map2_c>::from_shape(32, 32).unwrap();
                let (dst1, dst2): (Mat<$dst1_t, $dst1_c>, _) = map1.convert_maps(&map2).unwrap();
                assert_eq!(dst1.data_type().unwrap().bits(), $code1);
                assert_eq!(dst1.cols(), map1.cols());
                assert_eq!(dst1.rows(), map1.rows());
                assert_eq!(dst2.data_type().unwrap().bits(), $code2);
                assert_eq!(dst2.cols(), map1.cols());
                assert_eq!(dst2.rows(), map1.rows());
            }
        };
    }
    impl_convert_maps_test!(
        convert_maps_16sc2_16sc1_to_16sc2_test,
        [i16, 2],
        [i16, 1],
        [i16, 2],
        [u16, 1],
        DataTypes::CV_16SC2.bits(),
        DataTypes::CV_16UC1.bits()
    );
    impl_convert_maps_test!(
        convert_maps_16sc2_16ic1_to_32fc1_test,
        [i16, 2],
        [i16, 1],
        [f32, 1],
        [f32, 1],
        DataTypes::CV_32FC1.bits(),
        DataTypes::CV_32FC1.bits()
    );

    impl_convert_maps_test!(
        convert_maps_16sc2_16ic1_to_32fc2_test,
        [i16, 2],
        [i16, 1],
        [f32, 2],
        DataTypes::CV_32FC2.bits()
    );

    impl_convert_maps_test!(
        convert_maps_16sc2_16uc1_to_16sc2_test,
        [i16, 2],
        [u16, 1],
        [i16, 2],
        [u16, 1],
        DataTypes::CV_16SC2.bits(),
        DataTypes::CV_16UC1.bits()
    );
    impl_convert_maps_test!(
        convert_maps_16sc2_16uc1_to_32fc1_test,
        [i16, 2],
        [u16, 1],
        [f32, 1],
        [f32, 1],
        DataTypes::CV_32FC1.bits(),
        DataTypes::CV_32FC1.bits()
    );
    impl_convert_maps_test!(
        convert_maps_16sc2_16uc1_to_32fc2_test,
        [i16, 2],
        [u16, 1],
        [f32, 2],
        DataTypes::CV_32FC2.bits()
    );

    impl_convert_maps_test!(
        convert_maps_16sc1_16sc2_to_16sc2_test,
        [i16, 1],
        [i16, 2],
        [i16, 2],
        [u16, 1],
        DataTypes::CV_16SC2.bits(),
        DataTypes::CV_16UC1.bits()
    );
    impl_convert_maps_test!(
        convert_maps_16sc1_16sc2_to_32fc1_test,
        [i16, 1],
        [i16, 2],
        [f32, 1],
        [f32, 1],
        DataTypes::CV_32FC1.bits(),
        DataTypes::CV_32FC1.bits()
    );
    impl_convert_maps_test!(
        convert_maps_16sc1_16sc2_to_32fc2_test,
        [i16, 1],
        [i16, 2],
        [f32, 2],
        DataTypes::CV_32FC2.bits()
    );
    impl_convert_maps_test!(
        convert_maps_16uc1_16sc2_to_16sc2_test,
        [u16, 1],
        [i16, 2],
        [i16, 2],
        [u16, 1],
        DataTypes::CV_16SC2.bits(),
        DataTypes::CV_16UC1.bits()
    );
    impl_convert_maps_test!(
        convert_maps_16uc1_16sc2_to_32fc1_test,
        [u16, 1],
        [i16, 2],
        [f32, 1],
        [f32, 1],
        DataTypes::CV_32FC1.bits(),
        DataTypes::CV_32FC1.bits()
    );
    impl_convert_maps_test!(
        convert_maps_16uc1_16sc2_to_32fc2_test,
        [u16, 1],
        [i16, 2],
        [f32, 2],
        DataTypes::CV_32FC2.bits()
    );
    impl_convert_maps_test!(
        convert_maps_32fc1_32fc1_to_16sc2_test,
        [f32, 1],
        [f32, 1],
        [i16, 2],
        [u16, 1],
        DataTypes::CV_16SC2.bits(),
        DataTypes::CV_16UC1.bits()
    );
    impl_convert_maps_test!(
        convert_maps_32fc1_32fc1_to_32fc1_test,
        [f32, 1],
        [f32, 1],
        [f32, 1],
        [f32, 1],
        DataTypes::CV_32FC1.bits(),
        DataTypes::CV_32FC1.bits()
    );

    impl_convert_maps_test!(
        convert_maps_32fc1_32fc1_to_32fc2_test,
        [f32, 1],
        [f32, 1],
        [f32, 2],
        DataTypes::CV_32FC2.bits()
    );
    macro_rules! impl_convert_maps_with_interpolation_test {
        ($name:ident, [$map1_t:ty, $map1_c:tt], [$map2_t:ty, $map2_c:tt],[$dst_t:ty, $dst_c:tt], $code:expr) => {
            #[test]
            fn $name() {
                let map1 = Mat::<$map1_t, $map1_c>::from_shape(32, 32).unwrap();
                let map2 = Mat::<$map2_t, $map2_c>::from_shape(32, 32).unwrap();
                let dst: Mat<$dst_t, $dst_c> = map1.convert_maps_with_interpolation(&map2).unwrap();
                assert_eq!(dst.data_type().unwrap().bits(), $code);
                assert_eq!(dst.cols(), map1.cols());
                assert_eq!(dst.rows(), map1.rows());
            }
        };
    }

    impl_convert_maps_with_interpolation_test!(
        convert_maps_with_interpolation_16sc2_16sc1_to_16sc2_test,
        [i16, 2],
        [i16, 1],
        [i16, 2],
        DataTypes::CV_16SC2.bits()
    );
    impl_convert_maps_with_interpolation_test!(
        convert_maps_with_interpolation_16sc2_16sc1_to_32fc2_test,
        [i16, 2],
        [i16, 1],
        [f32, 2],
        DataTypes::CV_32FC2.bits()
    );
    impl_convert_maps_with_interpolation_test!(
        convert_maps_with_interpolation_16sc2_16uc1_to_16sc2_test,
        [i16, 2],
        [u16, 1],
        [i16, 2],
        DataTypes::CV_16SC2.bits()
    );
    impl_convert_maps_with_interpolation_test!(
        convert_maps_with_interpolation_16sc2_16uc1_to_32fc2_test,
        [i16, 2],
        [u16, 1],
        [f32, 2],
        DataTypes::CV_32FC2.bits()
    );
    impl_convert_maps_with_interpolation_test!(
        convert_maps_with_interpolation_16sc1_16sc2_to_16sc2_test,
        [i16, 1],
        [i16, 2],
        [i16, 2],
        DataTypes::CV_16SC2.bits()
    );
    impl_convert_maps_with_interpolation_test!(
        convert_maps_with_interpolation_16sc1_16sc2_to_32fc2_test,
        [i16, 1],
        [i16, 2],
        [f32, 2],
        DataTypes::CV_32FC2.bits()
    );
    impl_convert_maps_with_interpolation_test!(
        convert_maps_with_interpolation_16uc1_16sc2_to_16sc2_test,
        [u16, 1],
        [i16, 2],
        [i16, 2],
        DataTypes::CV_16SC2.bits()
    );
    impl_convert_maps_with_interpolation_test!(
        convert_maps_with_interpolation_16uc1_16sc2_to_32fc2_test,
        [u16, 1],
        [i16, 2],
        [f32, 2],
        DataTypes::CV_32FC2.bits()
    );
    impl_convert_maps_with_interpolation_test!(
        convert_maps_with_interpolation_32fc1_32fc1_to_16sc2_test,
        [f32, 1],
        [f32, 1],
        [i16, 2],
        DataTypes::CV_16SC2.bits()
    );
    impl_convert_maps_with_interpolation_test!(
        convert_maps_with_interpolation_32fc1_32fc1_to_32fc1_test,
        [f32, 1],
        [f32, 1],
        [f32, 1],
        DataTypes::CV_32FC1.bits()
    );
    impl_convert_maps_with_interpolation_test!(
        convert_maps_with_interpolation_32fc1_32fc1_to_32fc2_test,
        [f32, 1],
        [f32, 1],
        [f32, 2],
        DataTypes::CV_32FC2.bits()
    );

    #[test]
    fn convert_maps_16sc2_test() {
        let map = Mat::<f32, 2>::from_shape(32, 32).unwrap();
        let (dst1, dst2) = map.convert_maps_16sc2().unwrap();
        assert_eq!(dst1.data_type().unwrap().bits(), DataTypes::CV_16SC2.bits());
        assert_eq!(dst1.cols(), map.cols());
        assert_eq!(dst1.rows(), map.rows());
        assert_eq!(dst2.data_type().unwrap().bits(), DataTypes::CV_16UC1.bits());
        assert_eq!(dst2.cols(), map.cols());
        assert_eq!(dst2.rows(), map.rows());
    }
    #[test]
    fn convert_maps_32fc1_test() {
        let map = Mat::<f32, 2>::from_shape(32, 32).unwrap();
        let (dst1, dst2) = map.convert_maps_32fc1().unwrap();
        assert_eq!(dst1.data_type().unwrap().bits(), DataTypes::CV_32FC1.bits());
        assert_eq!(dst1.cols(), map.cols());
        assert_eq!(dst1.rows(), map.rows());
        assert_eq!(dst2.data_type().unwrap().bits(), DataTypes::CV_32FC1.bits());
        assert_eq!(dst2.cols(), map.cols());
        assert_eq!(dst2.rows(), map.rows());
    }
    #[test]
    fn convert_maps_32fc2_test() {
        let map = Mat::<f32, 2>::from_shape(32, 32).unwrap();
        let dst = map.convert_maps_32fc2().unwrap();
        assert_eq!(dst.data_type().unwrap().bits(), DataTypes::CV_32FC2.bits());
        assert_eq!(dst.cols(), map.cols());
        assert_eq!(dst.rows(), map.rows());
    }
    #[test]
    fn convert_maps_16sc2_with_interpolation_test() {
        let map = Mat::<f32, 2>::from_shape(32, 32).unwrap();
        let dst = map.convert_maps_16sc2_with_interpolation().unwrap();
        assert_eq!(dst.data_type().unwrap().bits(), DataTypes::CV_16SC2.bits());
        assert_eq!(dst.cols(), map.cols());
        assert_eq!(dst.rows(), map.rows());
    }
    #[test]
    fn convert_maps_32fc1_with_interpolation_test() {
        let map = Mat::<f32, 2>::from_shape(32, 32).unwrap();
        let dst = map.convert_maps_32fc1_with_interpolation().unwrap();
        assert_eq!(dst.data_type().unwrap().bits(), DataTypes::CV_32FC1.bits());
        assert_eq!(dst.cols(), map.cols());
        assert_eq!(dst.rows(), map.rows());
    }
    #[test]
    fn convert_maps_32fc2_with_interpolation_test() {
        let map = Mat::<f32, 2>::from_shape(32, 32).unwrap();
        let dst = map.convert_maps_32fc2_with_interpolation().unwrap();
        assert_eq!(dst.data_type().unwrap().bits(), DataTypes::CV_32FC2.bits());
        assert_eq!(dst.cols(), map.cols());
        assert_eq!(dst.rows(), map.rows());
    }
}
