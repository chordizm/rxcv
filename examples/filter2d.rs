use rxcv::{imgcodecs::Decode, imgproc::Filter2D, BorderTypes, Mat};

fn main() {
    let data = include_bytes!("../mock/lenna.png");
    let src = Mat::<u8, 3>::decode(data).unwrap();
    let kernel_data =
        ndarray::Array::from_shape_vec((3, 3, 1), vec![0., 1., 0., 1., -5., 1., 0., 1., 0.])
            .unwrap();
    let kernel = Mat::<f64, 1>::from(&kernel_data);
    let dst = src
        .filter2d(kernel, -1, -1, 0., BorderTypes::BORDER_DEFAULT)
        .unwrap();
    println!(
        "Result image: W:{}, H:{}, C:{}",
        dst.cols(),
        dst.rows(),
        dst.channels()
    );
}
