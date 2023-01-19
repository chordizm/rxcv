#![allow(non_upper_case_globals)]
use bitflags::bitflags;

bitflags! {
    pub struct ColorConversionCodes: i32 {
        const COLOR_BGR2BGRA = 0;
        const COLOR_RGB2RGBA = Self::COLOR_BGR2BGRA.bits;

        const COLOR_BGRA2BGR = 1;
        const COLOR_RGBA2RGB = Self::COLOR_BGRA2BGR.bits;

        const COLOR_BGR2RGBA = 2;
        const COLOR_RGB2BGRA = Self::COLOR_BGR2RGBA.bits;

        const COLOR_RGBA2BGR = 3;
        const COLOR_BGRA2RGB = Self::COLOR_RGBA2BGR.bits;

        const COLOR_BGR2RGB = 4;
        const COLOR_RGB2BGR = Self::COLOR_BGR2RGB.bits;

        const COLOR_BGRA2RGBA = 5;
        const COLOR_RGBA2BGRA = Self::COLOR_BGRA2RGBA.bits;

        const COLOR_BGR2GRAY = 6;
        const COLOR_RGB2GRAY = 7;
        const COLOR_GRAY2BGR = 8;
        const COLOR_GRAY2RGB = Self::COLOR_GRAY2BGR.bits;
        const COLOR_GRAY2BGRA = 9;
        const COLOR_GRAY2RGBA = Self::COLOR_GRAY2BGRA.bits;
        const COLOR_BGRA2GRAY = 10;
        const COLOR_RGBA2GRAY = 11;

        const COLOR_BGR2BGR565 = 12;
        const COLOR_RGB2BGR565 = 13;
        const COLOR_BGR5652BGR = 14;
        const COLOR_BGR5652RGB = 15;
        const COLOR_BGRA2BGR565 = 16;
        const COLOR_RGBA2BGR565 = 17;
        const COLOR_BGR5652BGRA = 18;
        const COLOR_BGR5652RGBA = 19;

        const COLOR_GRAY2BGR565 = 20;
        const COLOR_BGR5652GRAY = 21;

        const COLOR_BGR2BGR555 = 22;
        const COLOR_RGB2BGR555 = 23;
        const COLOR_BGR5552BGR = 24;
        const COLOR_BGR5552RGB = 25;
        const COLOR_BGRA2BGR555 = 26;
        const COLOR_RGBA2BGR555 = 27;
        const COLOR_BGR5552BGRA = 28;
        const COLOR_BGR5552RGBA = 29;

        const COLOR_GRAY2BGR555 = 30;
        const COLOR_BGR5552GRAY = 31;

        const COLOR_BGR2XYZ = 32;
        const COLOR_RGB2XYZ = 33;
        const COLOR_XYZ2BGR = 34;
        const COLOR_XYZ2RGB = 35;

        const COLOR_BGR2YCrCb = 36;
        const COLOR_RGB2YCrCb = 37;
        const COLOR_YCrCb2BGR = 38;
        const COLOR_YCrCb2RGB = 39;

        const COLOR_BGR2HSV = 40;
        const COLOR_RGB2HSV = 41;

        const COLOR_BGR2Lab = 44;
        const COLOR_RGB2Lab = 45;

        const COLOR_BGR2Luv = 50;
        const COLOR_RGB2Luv = 51;
        const COLOR_BGR2HLS = 52;
        const COLOR_RGB2HLS = 53;

        const COLOR_HSV2BGR = 54;
        const COLOR_HSV2RGB = 55;

        const COLOR_Lab2BGR = 56;
        const COLOR_Lab2RGB = 57;
        const COLOR_Luv2BGR = 58;
        const COLOR_Luv2RGB = 59;
        const COLOR_HLS2BGR = 60;
        const COLOR_HLS2RGB = 61;

        const COLOR_BGR2HSV_FULL = 66;
        const COLOR_RGB2HSV_FULL = 67;
        const COLOR_BGR2HLS_FULL = 68;
        const COLOR_RGB2HLS_FULL = 69;

        const COLOR_HSV2BGR_FULL = 70;
        const COLOR_HSV2RGB_FULL = 71;
        const COLOR_HLS2BGR_FULL = 72;
        const COLOR_HLS2RGB_FULL = 73;

        const COLOR_LBGR2Lab = 74;
        const COLOR_LRGB2Lab = 75;
        const COLOR_LBGR2Luv = 76;
        const COLOR_LRGB2Luv = 77;

        const COLOR_Lab2LBGR = 78;
        const COLOR_Lab2LRGB = 79;
        const COLOR_Luv2LBGR = 80;
        const COLOR_Luv2LRGB = 81;

        const COLOR_BGR2YUV = 82;
        const COLOR_RGB2YUV = 83;
        const COLOR_YUV2BGR = 84;
        const COLOR_YUV2RGB = 85;


        const COLOR_YUV2RGB_NV12 = 90;
        const COLOR_YUV2BGR_NV12 = 91;
        const COLOR_YUV2RGB_NV21 = 92;
        const COLOR_YUV2BGR_NV21 = 93;
        const COLOR_YUV420sp2RGB = Self::COLOR_YUV2RGB_NV21.bits;
        const COLOR_YUV420sp2BGR = Self::COLOR_YUV2BGR_NV21.bits;

        const COLOR_YUV2RGBA_NV12 = 94;
        const COLOR_YUV2BGRA_NV12 = 95;
        const COLOR_YUV2RGBA_NV21 = 96;
        const COLOR_YUV2BGRA_NV21 = 97;
        const COLOR_YUV420sp2RGBA = Self::COLOR_YUV2RGBA_NV21.bits;
        const COLOR_YUV420sp2BGRA = Self::COLOR_YUV2BGRA_NV21.bits;

        const COLOR_YUV2RGB_YV12 = 98;
        const COLOR_YUV2BGR_YV12 = 99;
        const COLOR_YUV2RGB_IYUV = 100;
        const COLOR_YUV2BGR_IYUV = 101;
        const COLOR_YUV2RGB_I420 = Self::COLOR_YUV2RGB_IYUV.bits;
        const COLOR_YUV2BGR_I420 = Self::COLOR_YUV2BGR_IYUV.bits;
        const COLOR_YUV420p2RGB = Self::COLOR_YUV2RGB_YV12.bits;
        const COLOR_YUV420p2BGR = Self::COLOR_YUV2BGR_YV12.bits;

        const COLOR_YUV2RGBA_YV12 = 102;
        const COLOR_YUV2BGRA_YV12 = 103;
        const COLOR_YUV2RGBA_IYUV = 104;
        const COLOR_YUV2BGRA_IYUV = 105;
        const COLOR_YUV2RGBA_I420 = Self::COLOR_YUV2RGBA_IYUV.bits;
        const COLOR_YUV2BGRA_I420 = Self::COLOR_YUV2BGRA_IYUV.bits;
        const COLOR_YUV420p2RGBA = Self::COLOR_YUV2RGBA_YV12.bits;
        const COLOR_YUV420p2BGRA = Self::COLOR_YUV2BGRA_YV12.bits;

        const COLOR_YUV2GRAY_420 = 106;
        const COLOR_YUV2GRAY_NV21 = Self::COLOR_YUV2GRAY_420.bits;
        const COLOR_YUV2GRAY_NV12 = Self::COLOR_YUV2GRAY_420.bits;
        const COLOR_YUV2GRAY_YV12 = Self::COLOR_YUV2GRAY_420.bits;
        const COLOR_YUV2GRAY_IYUV = Self::COLOR_YUV2GRAY_420.bits;
        const COLOR_YUV2GRAY_I420 = Self::COLOR_YUV2GRAY_420.bits;
        const COLOR_YUV420sp2GRAY = Self::COLOR_YUV2GRAY_420.bits;
        const COLOR_YUV420p2GRAY = Self::COLOR_YUV2GRAY_420.bits;


        const COLOR_YUV2RGB_UYVY = 107;
        const COLOR_YUV2BGR_UYVY = 108;
        //COLOR_YUV2RGB_VYUY = 109;
        //COLOR_YUV2BGR_VYUY = 110;
        const COLOR_YUV2RGB_Y422 = Self::COLOR_YUV2RGB_UYVY.bits;
        const COLOR_YUV2BGR_Y422 = Self::COLOR_YUV2BGR_UYVY.bits;
        const COLOR_YUV2RGB_UYNV = Self::COLOR_YUV2RGB_UYVY.bits;
        const COLOR_YUV2BGR_UYNV = Self::COLOR_YUV2BGR_UYVY.bits;

        const COLOR_YUV2RGBA_UYVY = 111;
        const COLOR_YUV2BGRA_UYVY = 112;
        //COLOR_YUV2RGBA_VYUY = 113;
        //COLOR_YUV2BGRA_VYUY = 114;
        const COLOR_YUV2RGBA_Y422 = Self::COLOR_YUV2RGBA_UYVY.bits;
        const COLOR_YUV2BGRA_Y422 = Self::COLOR_YUV2BGRA_UYVY.bits;
        const COLOR_YUV2RGBA_UYNV = Self::COLOR_YUV2RGBA_UYVY.bits;
        const COLOR_YUV2BGRA_UYNV = Self::COLOR_YUV2BGRA_UYVY.bits;

        const COLOR_YUV2RGB_YUY2 = 115;
        const COLOR_YUV2BGR_YUY2 = 116;
        const COLOR_YUV2RGB_YVYU = 117;
        const COLOR_YUV2BGR_YVYU = 118;
        const COLOR_YUV2RGB_YUYV = Self::COLOR_YUV2RGB_YUY2.bits;
        const COLOR_YUV2BGR_YUYV = Self::COLOR_YUV2BGR_YUY2.bits;
        const COLOR_YUV2RGB_YUNV = Self::COLOR_YUV2RGB_YUY2.bits;
        const COLOR_YUV2BGR_YUNV = Self::COLOR_YUV2BGR_YUY2.bits;

        const COLOR_YUV2RGBA_YUY2 = 119;
        const COLOR_YUV2BGRA_YUY2 = 120;
        const COLOR_YUV2RGBA_YVYU = 121;
        const COLOR_YUV2BGRA_YVYU = 122;
        const COLOR_YUV2RGBA_YUYV = Self::COLOR_YUV2RGBA_YUY2.bits;
        const COLOR_YUV2BGRA_YUYV = Self::COLOR_YUV2BGRA_YUY2.bits;
        const COLOR_YUV2RGBA_YUNV = Self::COLOR_YUV2RGBA_YUY2.bits;
        const COLOR_YUV2BGRA_YUNV = Self::COLOR_YUV2BGRA_YUY2.bits;

        const COLOR_YUV2GRAY_UYVY = 123;
        const COLOR_YUV2GRAY_YUY2 = 124;
        //CV_YUV2GRAY_VYUY = CV_YUV2GRAY_UYVY;
        const COLOR_YUV2GRAY_Y422 = Self::COLOR_YUV2GRAY_UYVY.bits;
        const COLOR_YUV2GRAY_UYNV = Self::COLOR_YUV2GRAY_UYVY.bits;
        const COLOR_YUV2GRAY_YVYU = Self::COLOR_YUV2GRAY_YUY2.bits;
        const COLOR_YUV2GRAY_YUYV = Self::COLOR_YUV2GRAY_YUY2.bits;
        const COLOR_YUV2GRAY_YUNV = Self::COLOR_YUV2GRAY_YUY2.bits;

        const COLOR_RGBA2mRGBA = 125;
        const COLOR_mRGBA2RGBA = 126;


        const COLOR_RGB2YUV_I420 = 127;
        const COLOR_BGR2YUV_I420 = 128;
        const COLOR_RGB2YUV_IYUV = Self::COLOR_RGB2YUV_I420.bits;
        const COLOR_BGR2YUV_IYUV = Self::COLOR_BGR2YUV_I420.bits;

        const COLOR_RGBA2YUV_I420 = 129;
        const COLOR_BGRA2YUV_I420 = 130;
        const COLOR_RGBA2YUV_IYUV = Self::COLOR_RGBA2YUV_I420.bits;
        const COLOR_BGRA2YUV_IYUV = Self::COLOR_BGRA2YUV_I420.bits;
        const COLOR_RGB2YUV_YV12 = 131;
        const COLOR_BGR2YUV_YV12 = 132;
        const COLOR_RGBA2YUV_YV12 = 133;
        const COLOR_BGRA2YUV_YV12 = 134;


        const COLOR_BayerBG2BGR = 46;
        const COLOR_BayerGB2BGR = 47;
        const COLOR_BayerRG2BGR = 48;
        const COLOR_BayerGR2BGR = 49;

        const COLOR_BayerRGGB2BGR = Self::COLOR_BayerBG2BGR.bits;
        const COLOR_BayerGRBG2BGR = Self::COLOR_BayerGB2BGR.bits;
        const COLOR_BayerBGGR2BGR = Self::COLOR_BayerRG2BGR.bits;
        const COLOR_BayerGBRG2BGR = Self::COLOR_BayerGR2BGR.bits;

        const COLOR_BayerRGGB2RGB = Self::COLOR_BayerBGGR2BGR.bits;
        const COLOR_BayerGRBG2RGB = Self::COLOR_BayerGBRG2BGR.bits;
        const COLOR_BayerBGGR2RGB = Self::COLOR_BayerRGGB2BGR.bits;
        const COLOR_BayerGBRG2RGB = Self::COLOR_BayerGRBG2BGR.bits;

        const COLOR_BayerBG2RGB = Self::COLOR_BayerRG2BGR.bits;
        const COLOR_BayerGB2RGB = Self::COLOR_BayerGR2BGR.bits;
        const COLOR_BayerRG2RGB = Self::COLOR_BayerBG2BGR.bits;
        const COLOR_BayerGR2RGB = Self::COLOR_BayerGB2BGR.bits;

        const COLOR_BayerBG2GRAY = 86;
        const COLOR_BayerGB2GRAY = 87;
        const COLOR_BayerRG2GRAY = 88;
        const COLOR_BayerGR2GRAY = 89;

        const COLOR_BayerRGGB2GRAY = Self::COLOR_BayerBG2GRAY.bits;
        const COLOR_BayerGRBG2GRAY = Self::COLOR_BayerGB2GRAY.bits;
        const COLOR_BayerBGGR2GRAY = Self::COLOR_BayerRG2GRAY.bits;
        const COLOR_BayerGBRG2GRAY = Self::COLOR_BayerGR2GRAY.bits;


        const COLOR_BayerBG2BGR_VNG = 62;
        const COLOR_BayerGB2BGR_VNG = 63;
        const COLOR_BayerRG2BGR_VNG = 64;
        const COLOR_BayerGR2BGR_VNG = 65;

        const COLOR_BayerRGGB2BGR_VNG = Self::COLOR_BayerBG2BGR_VNG.bits;
        const COLOR_BayerGRBG2BGR_VNG = Self::COLOR_BayerGB2BGR_VNG.bits;
        const COLOR_BayerBGGR2BGR_VNG = Self::COLOR_BayerRG2BGR_VNG.bits;
        const COLOR_BayerGBRG2BGR_VNG = Self::COLOR_BayerGR2BGR_VNG.bits;

        const COLOR_BayerRGGB2RGB_VNG = Self::COLOR_BayerBGGR2BGR_VNG.bits;
        const COLOR_BayerGRBG2RGB_VNG = Self::COLOR_BayerGBRG2BGR_VNG.bits;
        const COLOR_BayerBGGR2RGB_VNG = Self::COLOR_BayerRGGB2BGR_VNG.bits;
        const COLOR_BayerGBRG2RGB_VNG = Self::COLOR_BayerGRBG2BGR_VNG.bits;

        const COLOR_BayerBG2RGB_VNG = Self::COLOR_BayerRG2BGR_VNG.bits;
        const COLOR_BayerGB2RGB_VNG = Self::COLOR_BayerGR2BGR_VNG.bits;
        const COLOR_BayerRG2RGB_VNG = Self::COLOR_BayerBG2BGR_VNG.bits;
        const COLOR_BayerGR2RGB_VNG = Self::COLOR_BayerGB2BGR_VNG.bits;


        const COLOR_BayerBG2BGR_EA = 135;
        const COLOR_BayerGB2BGR_EA = 136;
        const COLOR_BayerRG2BGR_EA = 137;
        const COLOR_BayerGR2BGR_EA = 138;

        const COLOR_BayerRGGB2BGR_EA = Self::COLOR_BayerBG2BGR_EA.bits;
        const COLOR_BayerGRBG2BGR_EA = Self::COLOR_BayerGB2BGR_EA.bits;
        const COLOR_BayerBGGR2BGR_EA = Self::COLOR_BayerRG2BGR_EA.bits;
        const COLOR_BayerGBRG2BGR_EA = Self::COLOR_BayerGR2BGR_EA.bits;

        const COLOR_BayerRGGB2RGB_EA = Self::COLOR_BayerBGGR2BGR_EA.bits;
        const COLOR_BayerGRBG2RGB_EA = Self::COLOR_BayerGBRG2BGR_EA.bits;
        const COLOR_BayerBGGR2RGB_EA = Self::COLOR_BayerRGGB2BGR_EA.bits;
        const COLOR_BayerGBRG2RGB_EA = Self::COLOR_BayerGRBG2BGR_EA.bits;

        const COLOR_BayerBG2RGB_EA = Self::COLOR_BayerRG2BGR_EA.bits;
        const COLOR_BayerGB2RGB_EA = Self::COLOR_BayerGR2BGR_EA.bits;
        const COLOR_BayerRG2RGB_EA = Self::COLOR_BayerBG2BGR_EA.bits;
        const COLOR_BayerGR2RGB_EA = Self::COLOR_BayerGB2BGR_EA.bits;


        const COLOR_BayerBG2BGRA = 139;
        const COLOR_BayerGB2BGRA = 140;
        const COLOR_BayerRG2BGRA = 141;
        const COLOR_BayerGR2BGRA = 142;

        const COLOR_BayerRGGB2BGRA = Self::COLOR_BayerBG2BGRA.bits;
        const COLOR_BayerGRBG2BGRA = Self::COLOR_BayerGB2BGRA.bits;
        const COLOR_BayerBGGR2BGRA = Self::COLOR_BayerRG2BGRA.bits;
        const COLOR_BayerGBRG2BGRA = Self::COLOR_BayerGR2BGRA.bits;

        const COLOR_BayerRGGB2RGBA = Self::COLOR_BayerBGGR2BGRA.bits;
        const COLOR_BayerGRBG2RGBA = Self::COLOR_BayerGBRG2BGRA.bits;
        const COLOR_BayerBGGR2RGBA = Self::COLOR_BayerRGGB2BGRA.bits;
        const COLOR_BayerGBRG2RGBA = Self::COLOR_BayerGRBG2BGRA.bits;

        const COLOR_BayerBG2RGBA = Self::COLOR_BayerRG2BGRA.bits;
        const COLOR_BayerGB2RGBA = Self::COLOR_BayerGR2BGRA.bits;
        const COLOR_BayerRG2RGBA = Self::COLOR_BayerBG2BGRA.bits;
        const COLOR_BayerGR2RGBA = Self::COLOR_BayerGB2BGRA.bits;

        const COLOR_COLORCVT_MAX = 143;
    }
}

bitflags! {
    pub struct ThresholdTypes: i32 {
        const THRESH_BINARY = 0;
        const THRESH_BINRAY_INV = 1;
        const THRESH_TRUNC = 2;
        const THRESH_TOZERO = 3;
        const THRESH_TOZERO_INV = 4;
        const THRESH_MASK = 7;
        const THRESH_OTSU = 8;
        const THRESH_TRIANGLE = 16;
    }
}

bitflags! {
    pub struct RetrievalModes: i32 {
        const RETR_EXTERNAL = 0;
        const RETR_LIST = 1;
        const RETR_CCOMP = 2;
        const RETR_TREE = 3;
        const RETR_FLOODFILL = 4;
    }
}

bitflags! {
    pub struct ContourApproximationModes: i32 {
        const CHAIN_APPROX_NONE = 1;
        const CHAIN_APPROX_SIMPLE = 2;
        const CHAIN_APPROX_TC89_L1 = 3;
        const CHAIN_APPROX_TC89_KCOS = 4;
    }
}
