#include <vector>
#include <opencv2/core.hpp>
#include <opencv2/imgcodecs.hpp>
#include "ffi.hpp"

using MatResult = FFIResult<cv::Mat *>;
using namespace std;

extern "C"
{
    FFIResult<int> cv_imwrite(cv::Mat *img, const char *path)
    {
        return try_execute<int>([&]()
                                { cv::imwrite(path, *img); return 0; },
                                -1);
    }

    MatResult cv_imread(const char *path, int flags)
    {
        return try_execute<cv::Mat *>([&]()
                                      { return new cv::Mat(cv::imread(path, flags)); },
                                      nullptr);
    }

    FFIResult<int> cv_imencode(cv::Mat *img, std::vector<uchar> *dst, char *ext)
    {
        return try_execute<int>([&]()
                                { cv::imencode(ext, *img, *dst); return 0; },
                                -1);
    }

    MatResult cv_imdecode(uchar *data, int size, int flags)
    {
        vector<uchar> bytes(data, data + size);
        return try_execute<cv::Mat *>([&]()
                                      { return new cv::Mat(cv::imdecode(bytes, flags)); },
                                      nullptr);
    }

    std::vector<uchar> *cv_new_bytes()
    {
        return new std::vector<uchar>();
    }

    uchar *cv_bytes_data(std::vector<uchar> *pointer)
    {
        return pointer->data();
    }

    std::size_t cv_bytes_size(std::vector<uchar> *pointer)
    {
        return pointer->size();
    }

    void cv_release_bytes(std::vector<uchar> *pointer)
    {
        delete pointer;
    }
}