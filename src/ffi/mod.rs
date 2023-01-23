#[repr(C)]
pub struct FFIResult<T> {
    pub ok: T,
    pub error: *const std::ffi::c_char,
}
