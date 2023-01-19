use std::{env, path::PathBuf};

fn get_files() -> std::io::Result<Vec<PathBuf>> {
    let dir = std::env::current_dir()?.join("lib");
    let entries = std::fs::read_dir(&dir);
    let paths = entries?
        .filter_map(|entry| entry.map(|e| e.path()).ok())
        .map(|p| dir.join(p))
        .collect::<Vec<PathBuf>>();
    Ok(paths)
}

fn main() {
    let mut build = cc::Build::new();
    for file in get_files().unwrap() {
        build.file(&file);
        println!("cargo:rerun-if-changed={}", file.to_str().unwrap());
    }
    build
        .cpp(true)
        .warnings(true)
        .flag("--std=c++17")
        .flag("-v")
        .flag("-g")
        .include(env::var("OPENCV_INCLUDE_DIR").unwrap_or("/usr/include/opencv4".to_string()))
        .compile("rxcv");
    println!(
        "cargo:rustc-link-search=native=/usr/lib/{}-linux-gnu",
        env::var("CARGO_CFG_TARGET_ARCH").unwrap()
    );
    println!("cargo:rustc-link-lib=dylib=opencv_core");
    println!("cargo:rustc-link-lib=dylib=opencv_imgcodecs");
    println!("cargo:rustc-link-lib=dylib=opencv_imgproc");
}
