mod ffi {
    use super::BytesPointer;

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_new_bytes() -> *const BytesPointer;
        pub(super) fn cv_release_bytes(pointer: *const BytesPointer);
        pub(super) fn cv_bytes_data(pointer: *const BytesPointer) -> *const u8;
        pub(super) fn cv_bytes_size(pointer: *const BytesPointer) -> usize;
    }
}

pub(crate) enum BytesPointer {}

pub struct Bytes {
    pub(crate) pointer: *const BytesPointer,
}

impl Bytes {
    pub fn data(&self) -> &[u8] {
        unsafe {
            let data = ffi::cv_bytes_data(self.pointer);
            let size = ffi::cv_bytes_size(self.pointer);
            std::slice::from_raw_parts(data, size)
        }
    }

    pub fn size(&self) -> usize {
        unsafe { ffi::cv_bytes_size(self.pointer) }
    }
}

impl Default for Bytes {
    fn default() -> Self {
        Self {
            pointer: unsafe { ffi::cv_new_bytes() },
        }
    }
}

impl Drop for Bytes {
    fn drop(&mut self) {
        unsafe { ffi::cv_release_bytes(self.pointer) }
    }
}
