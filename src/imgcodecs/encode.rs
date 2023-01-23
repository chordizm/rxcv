use crate::{
    core::{Bytes, Mat},
    result::Result,
};

mod ffi {
    use crate::{
        core::{BytesPointer, MatPointer},
        ffi::FFIResult,
    };

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_imencode(
            src: *const MatPointer,
            dst: *const BytesPointer,
            ext: *const std::ffi::c_char,
        ) -> FFIResult<i32>;
    }
}

impl<T, const C: usize> Mat<T, C> {
    pub fn encode(&self, ext: Ext) -> Result<Bytes> {
        let ext = std::ffi::CString::new(ext.to_string()).unwrap();
        let bytes = Bytes::default();
        Result::from(unsafe { ffi::cv_imencode(self.pointer, bytes.pointer, ext.as_ptr()) })?;
        Ok(bytes)
    }
}

pub enum Ext {
    PNG,
    JPG,
}

impl std::string::ToString for Ext {
    fn to_string(&self) -> String {
        match self {
            Ext::PNG => ".png".to_string(),
            Ext::JPG => ".jpg".to_string(),
        }
    }
}

impl<const C: usize> Mat<u8, C> {
    pub fn encode_png(&self) -> Result<Bytes> {
        self.encode(Ext::PNG)
    }

    pub fn encode_jpg(&self) -> Result<Bytes> {
        self.encode(Ext::JPG)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imencode_test() {
        let a = Mat::mock_7x6_square_5x4();
        let encoded = a.encode_png().unwrap();
        let b = Mat::<u8, 3>::decode(encoded.data()).unwrap();
        assert_eq!(a.cols(), b.cols());
        assert_eq!(a.rows(), b.rows());
        assert_eq!(a.data_type(), b.data_type());
        assert_eq!(a.data(), b.data());
    }
}
