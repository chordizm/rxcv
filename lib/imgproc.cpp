#include <vector>
#include <opencv2/core.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/imgcodecs.hpp>
#include "ffi.hpp"

using namespace std;

typedef std::vector<std::vector<cv::Point>> Contours;
typedef std::vector<cv::Point> Contour;

template <typename T>
struct Point_t
{
    T x;
    T y;
};

template <typename T>
struct Size_t
{
    T width;
    T height;
};
typedef Point_t<int> Point2i;
typedef Point2i Point;
typedef Point_t<float> Point2f;
typedef Size_t<int> Size2i;
typedef Size2i Size;

// ImageFiltering
extern "C"
{
    FFIResult<int> cv_bilateral_filter(cv::Mat *src, cv::Mat *dst, int d, double sigmaColor, double sigmaSpace, int borderType)
    {
        return try_execute<int>([&]()
                                { cv::bilateralFilter(*src, *dst, d, sigmaColor, sigmaSpace, borderType); return 0; },
                                -1);
    }

    FFIResult<int> cv_blur(cv::Mat *src, cv::Mat *dst, Size ksize, Point anchor, int borderType)
    {
        return try_execute<int>([&]()
                                { cv::blur(*src, *dst, cv::Size(ksize.width, ksize.height), cv::Point(anchor.x, anchor.y), borderType); return 0; },
                                -1);
    }

    FFIResult<int> cv_box_filter(cv::Mat *src, cv::Mat *dst, int ddepth, Size ksize, Point anchor, bool normalize, int borderType)
    {
        return try_execute<int>([&]()
                                { cv::boxFilter(*src, *dst, ddepth, cv::Size(ksize.width, ksize.height), cv::Point(anchor.x, anchor.y), normalize, borderType); return 0; },
                                -1);
    }

    // TODO: const Scalar & 	borderValue = morphologyDefaultBorderValue()
    FFIResult<int> cv_dilate(cv::Mat *src, cv::Mat *dst, cv::Mat *kernel, Point anchor, int iterations, int borderType)
    {
        return try_execute<int>([&]()
                                { cv::dilate(*src, *dst, *kernel, cv::Point(anchor.x, anchor.y), iterations, borderType); return 0; },
                                -1);
    }

    // TODO: const Scalar & 	borderValue = morphologyDefaultBorderValue()
    FFIResult<int> cv_erode(cv::Mat *src, cv::Mat *dst, cv::Mat *kernel, Point anchor, int iterations, int borderType)
    {
        return try_execute<int>([&]()
                                { cv::erode(*src, *dst, *kernel, cv::Point(anchor.x, anchor.y), iterations, borderType); return 0; },
                                -1);
    }

    FFIResult<int> cv_filter2d(cv::Mat *src, cv::Mat *dst, int ddepth, cv::Mat *kernel, int anchorX, int anchorY, double delta, int borderType)
    {
        return try_execute<int>([&]()
                                { cv::filter2D(*src, *dst, ddepth, *kernel, cv::Point(anchorX, anchorY), delta, borderType); return 0; },
                                -1);
    }

    FFIResult<int> cv_gaussian_blur(cv::Mat *src, cv::Mat *dst, Size ksize, double sigma_x, double sigma_y, int borderType)
    {
        return try_execute<int>([&]()
                                { cv::GaussianBlur(*src, *dst, cv::Size(ksize.width, ksize.height), sigma_x, sigma_y, borderType); return 0; },
                                -1);
    }

    FFIResult<int> cv_laplacian(cv::Mat *src, cv::Mat *dst, int ddepth, int ksize, double scale, double delta, int borderType)
    {
        return try_execute<int>([&]()
                                { cv::Laplacian(*src, *dst ,ddepth, ksize, scale, delta, borderType); return 0; },
                                -1);
    }

    FFIResult<int> cv_median_blur(cv::Mat *src, cv::Mat *dst, int ksize)
    {
        return try_execute<int>([&]()
                                { cv::medianBlur(*src, *dst, ksize); return 0; },
                                -1);
    }

    // TODO: const Scalar & 	borderValue = morphologyDefaultBorderValue()
    FFIResult<int> cv_morphology_ex(cv::Mat *src, cv::Mat *dst, int op, cv::Mat *kernel, Point anchor, int iterations, int border_type)
    {
        return try_execute<int>([&]()
                                { cv::morphologyEx(*src, *dst, op, *kernel, cv::Point(anchor.x, anchor.y), iterations, border_type); return 0; },
                                -1);
    }

    FFIResult<int> cv_pyr_down(cv::Mat *src, cv::Mat *dst, Size dstsize, int border_type)
    {
        return try_execute<int>([&]()
                                { cv::pyrDown(*src, *dst, cv::Size(dstsize.width, dstsize.height), border_type); return 0; },
                                -1);
    }

    FFIResult<int> cv_pyr_up(cv::Mat *src, cv::Mat *dst, Size dstsize, int border_type)
    {
        return try_execute<int>([&]()
                                { cv::pyrUp(*src, *dst, cv::Size(dstsize.width, dstsize.height), border_type); return 0; },
                                -1);
    }

    FFIResult<int> cv_scharr(cv::Mat *src, cv::Mat *dst, int ddepth, int dx, int dy, double scale, double delta, int border_type)
    {
        return try_execute<int>([&]()
                                { cv::Scharr(*src, *dst, ddepth, dx, dy, scale, delta, border_type); return 0; },
                                -1);
    }

    FFIResult<int> cv_sep_filter2d(cv::Mat *src, cv::Mat *dst, int ddepth, cv::Mat *kernel_x, cv::Mat *kernel_y, Point anchor, double delta, int border_type)
    {
        return try_execute<int>([&]()
                                { cv::sepFilter2D(*src, *dst, ddepth, *kernel_x, *kernel_y, cv::Point(anchor.x, anchor.y), delta, border_type); return 0; },
                                -1);
    }

    FFIResult<int> cv_sobel(cv::Mat *src, cv::Mat *dst, int ddepth, int dx, int dy, int ksize, double scale, double delta, int border_type)
    {
        return try_execute<int>([&]()
                                { cv::Sobel(*src, *dst, ddepth, dx, dy, ksize, scale, delta, border_type); return 0; },
                                -1);
    }

    FFIResult<int> cv_spatial_gradient(cv::Mat *src, cv::Mat *dx, cv::Mat *dy, int ksize, int border_type)
    {
        return try_execute<int>([&]()
                                { cv::spatialGradient(*src, *dx, *dy, ksize, border_type); return 0; },
                                -1);
    }

    FFIResult<int> cv_sqr_box_filter(cv::Mat *src, cv::Mat *dst, int ddepth, Size ksize, Point anchor, bool normalize, int border_type)
    {
        return try_execute<int>([&]()
                                { cv::sqrBoxFilter(*src, *dst, ddepth, cv::Size(ksize.width, ksize.height), cv::Point(anchor.x, anchor.y), normalize, border_type); return 0; },
                                -1);
    }
}

// Geometric Image Transformations
extern "C"
{
    FFIResult<int> cv_convert_maps(cv::Mat *map1, cv::Mat *map2, cv::Mat *dst1, cv::Mat *dst2, int dstmap1type, bool nninterpolation)
    {
        return try_execute<int>([&]()
                                { cv::convertMaps(*map1, *map2, *dst1, *dst2, dstmap1type, nninterpolation); return 0; },
                                -1);
    }

    FFIResult<int> cv_get_rect_sub_pix(cv::Mat *src, Size patchSize, Point2f center, cv::Mat *patch, int patch_type)
    {
        return try_execute<int>([&]()
                                { cv::getRectSubPix(*src, cv::Size(patchSize.width, patchSize.height), cv::Point2f(center.x, center.y), *patch, patch_type); return 0; },
                                -1);
    }

    FFIResult<int> cv_invert_affine_transform(cv::Mat *src, cv::Mat *dst)
    {
        return try_execute<int>([&]()
                                { cv::invertAffineTransform(*src, *dst); return 0; },
                                -1);
    }

    // TODO: const Scalar & 	borderValue = morphologyDefaultBorderValue()
    FFIResult<int> cv_remap(cv::Mat *src, cv::Mat *dst, cv::Mat *map1, cv::Mat *map2, int interpolation, int border_mode)
    {
        return try_execute<int>([&]()
                                { cv::remap(*src, *dst, *map1, *map2, interpolation, border_mode); return 0; },
                                -1);
    }

    FFIResult<int> cv_resize(cv::Mat *src, cv::Mat *dst, Size size, double fx, double fy, int interpolation)
    {
        return try_execute<int>([&]()
                                { cv::resize(*src, *dst, cv::Size(size.width, size.height), fx, fy, interpolation); return 0; },
                                -1);
    }

    FFIResult<int> cv_warp_polar(cv::Mat *src, cv::Mat *dst, Size dsize, Point2f center, double max_radius, int flags)
    {
        return try_execute<int>([&]()
                                { cv::warpPolar(*src, *dst, cv::Size(dsize.width, dsize.height), cv::Point2f(center.x, center.y), max_radius, flags); return 0; },
                                -1);
    }
}

// Miscellaneous Image Transformations
extern "C"
{
    FFIResult<int> cv_adaptive_threshold(cv::Mat *src, cv::Mat *dst, double max_value, int adaptive_method, int threshold_type, int block_size, double c)
    {
        return try_execute<int>([&]()
                                { cv::adaptiveThreshold(*src, *dst, max_value, adaptive_method, threshold_type, block_size, c); return 0; },
                                -1);
    }
}

extern "C"
{
    FFIResult<int> cv_cvt_color(cv::Mat *src, cv::Mat *dst, int code)
    {
        return try_execute<int>([&]()
                                { cv::cvtColor(*src, *dst, code); return 0; },
                                -1);
    }

    FFIResult<double> cv_threshold(cv::Mat *src, cv::Mat *dst, int thresh, int maxval, int type)
    {
        return try_execute<double>([&]()
                                   { return cv::threshold(*src, *dst, thresh, maxval, type); },
                                   0);
    }

    FFIResult<int> cv_find_contours(cv::Mat *src, Contours *contours, int mode, int method)
    {
        return try_execute<int>([&]()
                                { cv::findContours(*src, *contours, mode, method); return 0; },
                                -1);
    }

    Contours *cv_new_contours()
    {
        return new Contours();
    }

    Contour *cv_new_contour()
    {
        return new Contour();
    }

    Contour *cv_contours_at(Contours *contours, int index)
    {
        return &contours->at(index);
    }

    int cv_contours_size(Contours *contours)
    {
        return contours->size();
    }

    int cv_contour_size(Contour *contour)
    {
        return contour->size();
    }

    Point cv_contour_at(Contour *contour, int index)
    {
        auto pt = contour->at(index);
        return Point{pt.x, pt.y};
    }

    void cv_release_contours(Contours *contours)
    {
        delete contours;
    }

    void cv_release_contour(Contour *contour)
    {
        delete contour;
    }

    double cv_contour_area(Contour *contour)
    {
        return cv::contourArea(*contour);
    }

    double cv_contour_arc_length(Contour *contour, bool closed)
    {
        return cv::arcLength(*contour, closed);
    }
}