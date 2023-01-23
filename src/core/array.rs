use super::Mat;
use crate::result::Result;
use ndarray::Dim;

impl<T, const C: usize> From<&Mat<T, C>> for ndarray::Array<T, Dim<[usize; 3]>>
where
    T: Clone,
{
    fn from(mat: &Mat<T, C>) -> Self {
        ndarray::Array::from_shape_vec(
            (
                mat.rows() as usize,
                mat.cols() as usize,
                mat.channels() as usize,
            ),
            mat.data().to_vec(),
        )
        .unwrap()
    }
}

macro_rules! impl_mat_from_ndarray {
    ($t:ty, $c:tt, $code:expr) => {
        impl<'a> Mat<$t, $c> {
            pub fn from_ndarray(array: &'a ndarray::Array<$t, Dim<[usize; 3]>>) -> Result<Self> {
                let (rows, cols, _channels) = array.dim();
                Self::from_shape_vec(rows, cols, array.as_slice().unwrap())
            }
        }
    };
}

impl_mat_from_ndarray!(u8, 1, DataTypes::CV_8U);
impl_mat_from_ndarray!(u8, 2, DataTypes::CV_8U);
impl_mat_from_ndarray!(u8, 3, DataTypes::CV_8U);
impl_mat_from_ndarray!(i8, 1, DataTypes::CV_8S);
impl_mat_from_ndarray!(i8, 2, DataTypes::CV_8S);
impl_mat_from_ndarray!(i8, 3, DataTypes::CV_8S);
impl_mat_from_ndarray!(u16, 1, DataTypes::CV_16U);
impl_mat_from_ndarray!(u16, 2, DataTypes::CV_16U);
impl_mat_from_ndarray!(u16, 3, DataTypes::CV_16U);
impl_mat_from_ndarray!(i16, 1, DataTypes::CV_16S);
impl_mat_from_ndarray!(i16, 2, DataTypes::CV_16S);
impl_mat_from_ndarray!(i16, 3, DataTypes::CV_16S);
impl_mat_from_ndarray!(i32, 1, DataTypes::CV_32S);
impl_mat_from_ndarray!(i32, 2, DataTypes::CV_32S);
impl_mat_from_ndarray!(i32, 3, DataTypes::CV_32S);
impl_mat_from_ndarray!(f32, 1, DataTypes::CV_32F);
impl_mat_from_ndarray!(f32, 2, DataTypes::CV_32F);
impl_mat_from_ndarray!(f32, 3, DataTypes::CV_32F);
impl_mat_from_ndarray!(f64, 1, DataTypes::CV_64F);
impl_mat_from_ndarray!(f64, 2, DataTypes::CV_64F);
impl_mat_from_ndarray!(f64, 3, DataTypes::CV_64F);

impl<T, const C: usize> Mat<T, C> {
    pub fn to_ndarray(&self) -> ndarray::Array<T, Dim<[usize; 3]>>
    where
        T: Clone,
    {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_test() {
        let src = Mat::mock_7x6_square_5x4();

        let a = src.to_ndarray();
        assert_eq!(
            (
                src.rows() as usize,
                src.cols() as usize,
                src.channels() as usize,
            ),
            a.dim()
        );
        assert_eq!(a.as_slice().unwrap(), src.data());

        let dst = Mat::<u8, 3>::from_ndarray(&a).unwrap();
        assert_eq!(
            (
                src.rows() as usize,
                src.cols() as usize,
                src.channels() as usize,
            ),
            (
                dst.rows() as usize,
                dst.cols() as usize,
                dst.channels() as usize,
            )
        );
        assert_eq!(dst.data(), src.data());
    }
}
