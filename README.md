# RxCV

RxCV provide Rust programming experience to OpenCV.

| Platform | Arch    | Status |
| :------- | :------ | :----- |
| Linux    | x86_64  | 0.1.0  |
| Linux    | aarch64 | 0.1.0  |
| Windows  | All     | TBA    |
| macOS    | All     | TBA    |

Currently support opencv installed from apt.

# Modules

| Module     | Status     |
| :--------- | :--------- |
| core       | [WIP]0.1.0 |
| imgproc    | [WIP]0.1.0 |
| imgcodecs  | [WIP]0.1.0 |
| videoio    | TBA        |
| calib3d    | TBA        |
| features2d | TBA        |
| objdetect  | TBA        |
| dnn        | TBA        |
| ml         | TBA        |
| flann      | TBA        |
| photo      | TBA        |
| stiching   | TBA        |
| gapi       | TBA        |

# Get started

```c
std::vector<uchar> data = /* Some binary */;
cv::Mat src = cv::imdecode(data, cv::IMREAD_COLOR), dst;

// Execution time error.
auto thresh = cv::threshold(src, dst, 0, 255, cv::THRESH_BINARY | cv::THRESH_OTSU);
```

```rust,ignore
use rxcv::{
    imgproc::{Threshold, ThresholdTypes},
    Mat,
};
let data:&[u8] = &[1, 2, 3, 4, 5, 6];
let src = Mat::<u8, 3>::decode(data).unwrap();
/**
Not implemented threshold on Mat::<u8, 3>
let (thresh, dst) = src.threshold(
        0,
        255,
        ThresholdTypes::THRESH_BINARY | ThresholdTypes::THRESH_OTSU
    );
*/
// Convert Color;
let src = src.cvt_color_bgr2gray().unwrap();
let (thresh, dst) = src.threshold(
        0,
        255,
        ThresholdTypes::THRESH_BINARY | ThresholdTypes::THRESH_OTSU
    ).unwrap();
```

# Example

```c
std::vector<uchar> data = /* Some binary */;
cv::Mat src = cv::imdecode(data);
```

```rust,ignore
use rxcv::Mat;
let data:&[u8] = &[1, 2, 3, 4, 5, 6];
let src = Mat::<u8, 3>::decode(data).unwrap();
```

# License

This project is licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT license](./LICENSE-MIT) at your option.
