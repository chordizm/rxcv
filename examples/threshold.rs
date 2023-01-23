use rxcv::{
    imgproc::{Threshold, ThresholdTypes},
    Mat,
};

fn main() {
    let data = include_bytes!("../mock/lenna.png");
    let src = Mat::<u8, 1>::decode(data).unwrap();
    let (thresh, dst) = src
        .threshold(
            0,
            255,
            ThresholdTypes::THRESH_BINARY | ThresholdTypes::THRESH_OTSU,
        )
        .unwrap();
    println!("Threshold: {}", thresh);
    println!(
        "Result image: W:{}, H:{}, C:{}",
        dst.cols(),
        dst.rows(),
        dst.channels()
    );
}
