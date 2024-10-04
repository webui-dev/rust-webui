#![allow(dead_code)]
#![allow(unused_variables)]

use reqwest;
use std::env;

const BASE_URL: &str = "https://github.com/webui-dev/webui/releases/download/2.5.0-beta.2/";

fn main() {
    // WebUI static lib

    const LINUX_ARM: &str = "webui-linux-gcc-arm";
    const LINUX_ARM64: &str = "webui-linux-gcc-arm64";
    const LINUX_X64: &str = "webui-linux-gcc-x64";
    const MACOS_ARM64: &str = "webui-macos-clang-arm64";
    const MACOS_X64: &str = "webui-macos-clang-x64";
    const WINDOWS_X64: &str = "webui-windows-msvc-x64";

    let out_dir = env::var("OUT_DIR").unwrap();
    
    let target;
    #[cfg(target_os = "linux")]
    {
        #[cfg(target_arch = "aarch64")]
        {
            target = LINUX_ARM64;
        }
        
        #[cfg(target_arch = "arm")]
        {
            target = LINUX_ARM;
        }

        #[cfg(target_arch = "x86_64")]
        {
            target = LINUX_X64;
        }
    }

    #[cfg(target_os = "windows")]
    {
        target = WINDOWS_X64;
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=shell32");
    }

    #[cfg(target_os = "macos")]
    {
        #[cfg(target_arch = "aarch64")]
        {
            target = MACOS_ARM64;
        }

        #[cfg(target_arch = "x86_64")]
        {
            target = MACOS_X64;
        }
    }

    download(target, &out_dir);

    println!("cargo:rustc-link-search=native={}/{}", out_dir, target);

    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:rustc-link-lib=webui-2-static");
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=dylib=webui-2");

        let src = format!("{}/{}/webui-2.dll", out_dir, target);
        let dst = format!("{}/../../../webui-2.dll", out_dir);

        std::fs::copy(src, dst).unwrap();
    }

}

fn download(target: &str, cache_dir: &str) {
    let url = format!("{}{}{}", BASE_URL, target, ".zip");

    // check if directory exists
    if let Err(_) = std::fs::create_dir_all(cache_dir) {
        return;
    }

    let response = reqwest::blocking::get(url).unwrap();
    let status = response.status();
    if !status.is_success() {
        panic!("Failed to download WebUI static lib: {}", status);
    }

    let zip = std::io::Cursor::new(response.bytes().unwrap());
    let mut archive = zip::ZipArchive::new(zip).unwrap();
    archive.extract(cache_dir).unwrap();

}