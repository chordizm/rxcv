use bitflags::bitflags;

bitflags! {
    pub struct ImreadModes: i32 {
       const IMREAD_UNCHANGED = -1;
       const IMREAD_GRAYSCALE = 0;
       const IMREAD_COLOR = 1;
       const IMREAD_ANYDEPTH = 2;
       const IMREAD_ANYCOLOR = 4;
       const IMREAD_LOAD_GDAL = 8;
       const IMREAD_REDUCED_GRAYSCALE_2 = 16;
       const IMREAD_REDUCED_COLOR_2 = 17;
       const IMREAD_REDUCED_GRAYSCALE_4 = 32;
       const IMREAD_REDUCED_COLOR_4 = 33;
       const IMREAD_REDUCED_GRAYSCALE_8 = 64;
       const IMREAD_REDUCED_COLOR_8 = 65;
       const IMREAD_IGNORE_ORIENTATION = 128;
    }
}
