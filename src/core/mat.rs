use std::marker::PhantomData;

use crate::result::Result;

use super::consts::*;

mod ffi {
    use super::MatPointer;
    use crate::ffi::FFIResult;

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_new_mat() -> FFIResult<*const MatPointer>;
        pub(super) fn cv_mat_ones(
            rows: i32,
            cols: i32,
            r#type: i32,
        ) -> FFIResult<*const MatPointer>;
        pub(super) fn cv_mat_from_shape(
            rows: i32,
            cols: i32,
            r#type: i32,
        ) -> FFIResult<*const MatPointer>;
        pub(super) fn cv_mat_from_shape_vec(
            rows: i32,
            cols: i32,
            r#type: i32,
            src: *const std::ffi::c_void,
        ) -> FFIResult<*const MatPointer>;
        pub(super) fn cv_mat_type(pointer: *const MatPointer) -> i32;
        pub(super) fn cv_mat_data(pointer: *const MatPointer) -> *const std::ffi::c_void;
        pub(super) fn cv_mat_size(pointer: *const MatPointer) -> i32;
        pub(super) fn cv_mat_cols(pointer: *const MatPointer) -> i32;
        pub(super) fn cv_mat_rows(pointer: *const MatPointer) -> i32;
        pub(super) fn cv_mat_channels(pointer: *const MatPointer) -> i32;
        pub(super) fn cv_release_mat(pointer: *const MatPointer);
    }
}

pub(crate) enum MatPointer {}

pub struct Mat<T, const C: usize> {
    pub(crate) pointer: *const MatPointer,
    data_type: PhantomData<T>,
}

macro_rules! impl_mat {
    ($t:ty, $c:tt, $code:expr) => {
        impl Mat<$t, $c> {
            pub fn from_shape(rows: usize, cols: usize) -> Result<Self> {
                let pointer = Result::<*const MatPointer>::from(unsafe {
                    ffi::cv_mat_from_shape(rows as i32, cols as i32, make_type($code, $c as i32))
                })?;
                Ok(Mat::<$t, $c>::from_ptr(pointer))
            }
            pub fn from_shape_vec(rows: usize, cols: usize, data: &[$t]) -> Result<Self> {
                let pointer = Result::<*const MatPointer>::from(unsafe {
                    ffi::cv_mat_from_shape_vec(
                        rows as i32,
                        cols as i32,
                        make_type($code, $c as i32),
                        data.as_ptr() as *const std::ffi::c_void,
                    )
                })?;
                Ok(Mat::<$t, $c>::from_ptr(pointer))
            }
            pub fn ones(rows: i32, cols: i32) -> Result<Self> {
                let pointer = Result::<*const MatPointer>::from(unsafe {
                    ffi::cv_mat_ones(rows as i32, cols as i32, make_type($code, $c as i32))
                })?;
                Ok(Mat::<$t, $c>::from_ptr(pointer))
            }
        }
    };
}

impl_mat!(u8, 1, DataTypes::CV_8U);
impl_mat!(u8, 2, DataTypes::CV_8U);
impl_mat!(u8, 3, DataTypes::CV_8U);
impl_mat!(i8, 1, DataTypes::CV_8S);
impl_mat!(i8, 2, DataTypes::CV_8S);
impl_mat!(i8, 3, DataTypes::CV_8S);
impl_mat!(u16, 1, DataTypes::CV_16U);
impl_mat!(u16, 2, DataTypes::CV_16U);
impl_mat!(u16, 3, DataTypes::CV_16U);
impl_mat!(i16, 1, DataTypes::CV_16S);
impl_mat!(i16, 2, DataTypes::CV_16S);
impl_mat!(i16, 3, DataTypes::CV_16S);
impl_mat!(i32, 1, DataTypes::CV_32S);
impl_mat!(i32, 2, DataTypes::CV_32S);
impl_mat!(i32, 3, DataTypes::CV_32S);
impl_mat!(f32, 1, DataTypes::CV_32F);
impl_mat!(f32, 2, DataTypes::CV_32F);
impl_mat!(f32, 3, DataTypes::CV_32F);
impl_mat!(f64, 1, DataTypes::CV_64F);
impl_mat!(f64, 2, DataTypes::CV_64F);
impl_mat!(f64, 3, DataTypes::CV_64F);

impl<T, const C: usize> Mat<T, C> {
    pub fn new() -> Result<Self> {
        let pointer = Result::from(unsafe { ffi::cv_new_mat() })?;
        Ok(Self::from_ptr(pointer))
    }
    pub(crate) fn from_ptr(pointer: *const MatPointer) -> Self {
        Self {
            pointer,
            data_type: PhantomData,
        }
    }

    pub fn data_type(&self) -> Option<DataTypes> {
        let r#type = unsafe { ffi::cv_mat_type(self.pointer) };
        DataTypes::from_bits(r#type)
    }

    pub fn data(&self) -> &[T] {
        unsafe {
            let data = ffi::cv_mat_data(self.pointer) as *const T;
            let size = ffi::cv_mat_size(self.pointer) as usize;
            std::slice::from_raw_parts(data, size)
        }
    }

    pub fn size(&self) -> i32 {
        unsafe { ffi::cv_mat_size(self.pointer) }
    }

    pub fn cols(&self) -> i32 {
        unsafe { ffi::cv_mat_cols(self.pointer) }
    }

    pub fn rows(&self) -> i32 {
        unsafe { ffi::cv_mat_rows(self.pointer) }
    }

    pub fn channels(&self) -> i32 {
        unsafe { ffi::cv_mat_channels(self.pointer) }
    }
}

impl<T, const C: usize> Drop for Mat<T, C> {
    fn drop(&mut self) {
        unsafe { ffi::cv_release_mat(self.pointer) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_shape_test() {
        let mat = Mat::<u8, 1>::from_shape(0, 0);
        assert_eq!(mat.as_ref().err(), None);
        let mat = mat.unwrap();
        assert_eq!(mat.cols(), 0);
        assert_eq!(mat.rows(), 0);
    }

    #[test]
    fn data_type_test() {
        let mat = Mat::mock_7x6_white();
        assert_eq!(mat.data_type(), Some(DataTypes::CV_8UC3));
    }

    #[test]
    fn data_test() {
        let mat = Mat::mock_7x6_white();
        let data = mat.data();
        assert_eq!(data.len(), 7 * 6 * 3);
        assert!(data.iter().all(|&v| v == 255));
    }

    #[test]
    fn size_test() {
        let mat = Mat::mock_7x6_white();
        assert_eq!(mat.size(), 7 * 6 * 3);
        assert_eq!(mat.cols(), 7);
        assert_eq!(mat.rows(), 6);
        assert_eq!(mat.channels(), 3);
    }

    #[test]
    fn ones_test1() {
        let mat = Mat::<u8, 1>::ones(4, 5).unwrap();
        assert_eq!(mat.size(), 4 * 5);
        assert_eq!(mat.cols(), 5);
        assert_eq!(mat.rows(), 4);
        assert_eq!(mat.channels(), 1);
        assert!(mat.data().iter().all(|&v| v == 1));
    }

    #[test]
    fn ones_test2() {
        let mat = Mat::<u8, 3>::ones(1, 1).unwrap();
        assert_eq!(mat.size(), 3);
        assert_eq!(mat.cols(), 1);
        assert_eq!(mat.rows(), 1);
        assert_eq!(mat.channels(), 3);
        assert_eq!(mat.data()[0], 1);
        assert_eq!(mat.data()[1], 0);
        assert_eq!(mat.data()[2], 0);
    }
}
