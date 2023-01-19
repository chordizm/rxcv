use std::marker::PhantomData;

use super::consts::*;

mod ffi {
    use super::MatPointer;

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_new_mat() -> *const MatPointer;
        pub(super) fn cv_mat_ones(rows: i32, cols: i32, r#type: i32) -> *const MatPointer;
        pub(super) fn cv_mat_from_shape(rows: i32, cols: i32, r#type: i32) -> *const MatPointer;
        pub(super) fn cv_mat_from_shape_vec(
            rows: i32,
            cols: i32,
            r#type: i32,
            src: *const std::ffi::c_void,
        ) -> *const MatPointer;
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

impl<const C: usize> From<(usize, usize)> for Mat<u8, C> {
    fn from((rows, cols): (usize, usize)) -> Self {
        Self::new(unsafe {
            ffi::cv_mat_from_shape(
                rows as i32,
                cols as i32,
                make_type(DataTypes::CV_8U, C as i32),
            )
        })
    }
}

impl<const C: usize> From<(usize, usize)> for Mat<i8, C> {
    fn from((rows, cols): (usize, usize)) -> Self {
        Self::new(unsafe {
            ffi::cv_mat_from_shape(
                rows as i32,
                cols as i32,
                make_type(DataTypes::CV_8S, C as i32),
            )
        })
    }
}

impl<const C: usize> From<(usize, usize)> for Mat<u16, C> {
    fn from((rows, cols): (usize, usize)) -> Self {
        Self::new(unsafe {
            ffi::cv_mat_from_shape(
                rows as i32,
                cols as i32,
                make_type(DataTypes::CV_16U, C as i32),
            )
        })
    }
}

impl<const C: usize> From<(usize, usize)> for Mat<i16, C> {
    fn from((rows, cols): (usize, usize)) -> Self {
        Self::new(unsafe {
            ffi::cv_mat_from_shape(
                rows as i32,
                cols as i32,
                make_type(DataTypes::CV_16S, C as i32),
            )
        })
    }
}

impl<const C: usize> From<(usize, usize)> for Mat<i32, C> {
    fn from((rows, cols): (usize, usize)) -> Self {
        Self::new(unsafe {
            ffi::cv_mat_from_shape(
                rows as i32,
                cols as i32,
                make_type(DataTypes::CV_32S, C as i32),
            )
        })
    }
}

impl<const C: usize> From<(usize, usize)> for Mat<f32, C> {
    fn from((rows, cols): (usize, usize)) -> Self {
        Self::new(unsafe {
            ffi::cv_mat_from_shape(
                rows as i32,
                cols as i32,
                make_type(DataTypes::CV_32F, C as i32),
            )
        })
    }
}

impl<const C: usize> From<(usize, usize)> for Mat<f64, C> {
    fn from((rows, cols): (usize, usize)) -> Self {
        Self::new(unsafe {
            ffi::cv_mat_from_shape(
                rows as i32,
                cols as i32,
                make_type(DataTypes::CV_64F, C as i32),
            )
        })
    }
}

impl<const C: usize> From<(usize, usize, &[u8])> for Mat<u8, C> {
    fn from((rows, cols, src): (usize, usize, &[u8])) -> Self {
        Self::new(unsafe {
            ffi::cv_mat_from_shape_vec(
                rows as i32,
                cols as i32,
                make_type(DataTypes::CV_8U, C as i32),
                src.as_ptr() as *const std::ffi::c_void,
            )
        })
    }
}

impl<const C: usize> From<(usize, usize, &[i8])> for Mat<i8, C> {
    fn from((rows, cols, src): (usize, usize, &[i8])) -> Self {
        Self::new(unsafe {
            ffi::cv_mat_from_shape_vec(
                rows as i32,
                cols as i32,
                make_type(DataTypes::CV_8S, C as i32),
                src.as_ptr() as *const std::ffi::c_void,
            )
        })
    }
}

impl<const C: usize> From<(usize, usize, &[u16])> for Mat<u16, C> {
    fn from((rows, cols, src): (usize, usize, &[u16])) -> Self {
        Self::new(unsafe {
            ffi::cv_mat_from_shape_vec(
                rows as i32,
                cols as i32,
                make_type(DataTypes::CV_16U, C as i32),
                src.as_ptr() as *const std::ffi::c_void,
            )
        })
    }
}

impl<const C: usize> From<(usize, usize, &[i16])> for Mat<i16, C> {
    fn from((rows, cols, src): (usize, usize, &[i16])) -> Self {
        Self::new(unsafe {
            ffi::cv_mat_from_shape_vec(
                rows as i32,
                cols as i32,
                make_type(DataTypes::CV_16S, C as i32),
                src.as_ptr() as *const std::ffi::c_void,
            )
        })
    }
}

impl<const C: usize> From<(usize, usize, &[i32])> for Mat<i32, C> {
    fn from((rows, cols, src): (usize, usize, &[i32])) -> Self {
        Self::new(unsafe {
            ffi::cv_mat_from_shape_vec(
                rows as i32,
                cols as i32,
                make_type(DataTypes::CV_32S, C as i32),
                src.as_ptr() as *const std::ffi::c_void,
            )
        })
    }
}

impl<const C: usize> From<(usize, usize, &[f32])> for Mat<f32, C> {
    fn from((rows, cols, src): (usize, usize, &[f32])) -> Self {
        Self::new(unsafe {
            ffi::cv_mat_from_shape_vec(
                rows as i32,
                cols as i32,
                make_type(DataTypes::CV_32F, C as i32),
                src.as_ptr() as *const std::ffi::c_void,
            )
        })
    }
}

impl<const C: usize> From<(usize, usize, &[f64])> for Mat<f64, C> {
    fn from((rows, cols, src): (usize, usize, &[f64])) -> Self {
        Self::new(unsafe {
            ffi::cv_mat_from_shape_vec(
                rows as i32,
                cols as i32,
                make_type(DataTypes::CV_64F, C as i32),
                src.as_ptr() as *const std::ffi::c_void,
            )
        })
    }
}

pub trait FromShape {
    fn from_shape(rows: i32, cols: i32) -> Self;
}

pub trait Ones {
    fn ones(rows: i32, cols: i32) -> Self;
}

macro_rules! impl_mat_initializer {
    ($t:ty, $c:tt, $code:expr) => {
        impl FromShape for Mat<$t, $c> {
            fn from_shape(rows: i32, cols: i32) -> Self {
                Self::new(unsafe { ffi::cv_mat_from_shape(rows, cols, $code.bits()) })
            }
        }

        impl Ones for Mat<$t, $c> {
            fn ones(rows: i32, cols: i32) -> Self {
                Self::new(unsafe { ffi::cv_mat_ones(rows, cols, $code.bits()) })
            }
        }
    };
}

impl_mat_initializer!(u8, 1, DataTypes::CV_8UC1);
impl_mat_initializer!(u8, 2, DataTypes::CV_8UC2);
impl_mat_initializer!(u8, 3, DataTypes::CV_8UC3);
impl_mat_initializer!(f32, 1, DataTypes::CV_32FC1);
impl_mat_initializer!(f32, 2, DataTypes::CV_32FC2);
impl_mat_initializer!(f32, 3, DataTypes::CV_32FC3);

impl<T, const C: usize> Mat<T, C> {
    pub(crate) fn new(pointer: *const MatPointer) -> Self {
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

impl<T, const C: usize> Default for Mat<T, C> {
    fn default() -> Self {
        Self::new(unsafe { ffi::cv_new_mat() })
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
        let mat = Mat::<u8, 1>::ones(4, 5);
        assert_eq!(mat.size(), 4 * 5);
        assert_eq!(mat.cols(), 5);
        assert_eq!(mat.rows(), 4);
        assert_eq!(mat.channels(), 1);
        assert!(mat.data().iter().all(|&v| v == 1));
    }

    #[test]
    fn ones_test2() {
        let mat = Mat::<u8, 3>::ones(1, 1);
        assert_eq!(mat.size(), 3);
        assert_eq!(mat.cols(), 1);
        assert_eq!(mat.rows(), 1);
        assert_eq!(mat.channels(), 3);
        assert_eq!(mat.data()[0], 1);
        assert_eq!(mat.data()[1], 0);
        assert_eq!(mat.data()[2], 0);
    }
}
