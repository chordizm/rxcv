use ndarray::Dim;

use crate::core::Mat;

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

impl<'a, T, const C: usize> From<&'a ndarray::Array<T, Dim<[usize; 3]>>> for Mat<T, C>
where
    T: 'a + Clone,
    Mat<T, C>: From<(usize, usize, &'a [T])>,
{
    fn from(array: &'a ndarray::Array<T, Dim<[usize; 3]>>) -> Self {
        let (rows, cols, _channels) = array.dim();
        Self::from((rows, cols, array.as_slice().unwrap()))
    }
}

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

        let dst = Mat::<u8, 3>::from(&a);
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
