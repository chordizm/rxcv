#include <vector>
#include <opencv2/core.hpp>
#include <opencv2/imgcodecs.hpp>

using namespace std;

extern "C"
{
    bool cv_imwrite(cv::Mat *img, const char *path)
    {
        try
        {
            return cv::imwrite(path, *img);
        }
        catch (std::exception &e)
        {
            return false;
        }
    }

    cv::Mat *cv_imread(const char *path, int flags)
    {
        try
        {
            auto img = cv::imread(path, flags);
            return new cv::Mat(img);
        }
        catch (std::exception &e)
        {
            return nullptr;
        }
    }

    bool cv_imencode(cv::Mat *img, std::vector<uchar> *dst, char *ext)
    {
        try
        {
            cv::imencode(ext, *img, *dst);
            return true;
        }
        catch (std::exception &e)
        {
            return false;
        }
    }

    cv::Mat *cv_imdecode(uchar *data, int size, int flags)
    {
        try
        {
            vector<uchar> bytes(data, data + size);
            auto img = cv::imdecode(bytes, flags);
            return new cv::Mat(img);
        }
        catch (std::exception &e)
        {
            return nullptr;
        }
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