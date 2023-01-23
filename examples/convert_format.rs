use rxcv::Mat;

fn main() {
    let data = include_bytes!("../mock/lenna.png");
    let src = Mat::<u8, 3>::decode(data).unwrap();
    println!(
        "Oritinal image: W:{}, H:{}, C:{}",
        src.cols(),
        src.rows(),
        src.channels()
    );
    let gray = src.cvt_color_bgr2gray().unwrap();
    println!(
        "Gray scale image: W:{}, H:{}, C:{}",
        gray.cols(),
        gray.rows(),
        gray.channels()
    );
    let hsv = src.cvt_color_bgr2hsv().unwrap();
    println!(
        "HSV: W:{}, H:{}, C:{}",
        hsv.cols(),
        hsv.rows(),
        hsv.channels()
    );
}
