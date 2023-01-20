#include <iostream>
#include <vector>
#include <opencv2/core.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/imgcodecs.hpp>

using namespace std;

typedef std::vector<std::vector<cv::Point>> Contours;
typedef std::vector<cv::Point> Contour;

extern "C"
{
    struct Point
    {
        int x;
        int y;
    };

    bool cv_cvt_color(cv::Mat *src, cv::Mat *dst, int code)
    {
        try
        {
            cv::cvtColor(*src, *dst, code);
            return true;
        }
        catch (std::exception &e)
        {
            return false;
        }
    }

    double cv_threshold(cv::Mat *src, cv::Mat *dst, int thresh, int maxval, int type)
    {
        try
        {
            return cv::threshold(*src, *dst, thresh, maxval, type);
        }
        catch (std::exception &e)
        {
            return -1.0;
        }
    }

    bool cv_filter2d(cv::Mat *src, cv::Mat *dst, int ddepth, cv::Mat *kernel, int anchorX, int anchorY, double delta, int borderType)
    {
        try
        {
            cv::filter2D(*src, *dst, ddepth, *kernel, cv::Point(anchorX, anchorY), delta, borderType);
            return true;
        }
        catch (std::exception &e)
        {
            return false;
        }
    }

    bool cv_median_blur(cv::Mat *src, cv::Mat *dst, int ksize)
    {
        try
        {
            cv::medianBlur(*src, *dst, ksize);
            return true;
        }
        catch (std::exception &e)
        {
            return false;
        }
    }

    bool cv_bilateral_filter(cv::Mat *src, cv::Mat *dst, int d, double sigmaColor, double sigmaSpace, int borderType)
    {
        try
        {
            cv::bilateralFilter(*src, *dst, d, sigmaColor, sigmaSpace, borderType);
            return true;
        }
        catch (std::exception &e)
        {
            return false;
        }
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

    bool cv_find_contours(cv::Mat *src, Contours *contours, int mode, int method)
    {
        try
        {
            cv::findContours(*src, *contours, mode, method);
            return true;
        }
        catch (std::exception &e)
        {
            return false;
        }
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