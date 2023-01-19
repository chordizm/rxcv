//! Reactive Programing Library for OpenCV
pub mod core;
pub mod imgcodecs;
pub mod imgproc;
pub mod ndarray;

pub use crate::core::*;

#[cfg(test)]
use imgcodecs::Decode;

#[cfg(test)]
impl Mat<u8, 3> {
    pub fn mock_7x6_white() -> Self {
        let data: &[u8] = include_bytes!("../mock/7x6_white.png");
        Self::decode(data).unwrap()
    }

    pub fn mock_7x6_square_5x4() -> Self {
        let data: &[u8] = include_bytes!("../mock/7x6_square_5x4.png");
        Self::decode(data).unwrap()
    }

    pub fn mock_7x6_square_2x4_2x3() -> Self {
        let data: &[u8] = include_bytes!("../mock/7x6_square_2x4_2x3.png");
        Self::decode(data).unwrap()
    }
}
