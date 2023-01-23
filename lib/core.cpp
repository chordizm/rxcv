#include <vector>
#include <opencv2/core.hpp>
#include "ffi.hpp"

using MatResult = FFIResult<cv::Mat *>;

using namespace std;

extern "C"
{
    MatResult cv_new_mat()
    {
        return try_execute<cv::Mat *>([&]()
                                      { return new cv::Mat(); },
                                      nullptr);
    }

    MatResult cv_mat_from_shape(int rows, int cols, int type)
    {
        return try_execute<cv::Mat *>([&]()
                                      { return new cv::Mat(cv::Size(cols, rows), type); },
                                      nullptr);
    }

    MatResult cv_mat_from_shape_vec(int rows, int cols, int type, void *data)
    {
        return try_execute<cv::Mat *>([&]()
                                      { return new cv::Mat(cv::Size(cols, rows), type, data); },
                                      nullptr);
    }

    MatResult cv_mat_ones(int rows, int cols, int type)
    {
        return try_execute<cv::Mat *>([&]()
                                      { return new cv::Mat(cv::Mat::ones(cv::Size(cols, rows), type)); },
                                      nullptr);
    }

    int cv_mat_type(cv::Mat *pointer)
    {
        return pointer->type();
    }

    uchar *cv_mat_data(cv::Mat *pointer)
    {
        return pointer->data;
    }

    int cv_mat_size(cv::Mat *pointer)
    {
        return pointer->cols * pointer->rows * pointer->channels();
    }

    int cv_mat_cols(cv::Mat *pointer)
    {
        return pointer->cols;
    }

    int cv_mat_rows(cv::Mat *pointer)
    {
        return pointer->rows;
    }

    int cv_mat_channels(cv::Mat *pointer)
    {
        return pointer->channels();
    }

    void cv_release_mat(cv::Mat *pointer)
    {
        delete pointer;
    }
}