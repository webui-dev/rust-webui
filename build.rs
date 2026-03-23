use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    let lib_filename = if target_os == "windows" {
        "webui-2-static.lib"
    } else {
        "libwebui-2-static.a"
    };

    let lib_path = Path::new(&manifest_dir).join(lib_filename);

    if !lib_path.exists() {
        let zip_name = select_zip(&target_os, &target_arch, &target_env);
        let url = format!(
            "https://github.com/webui-dev/webui/releases/download/nightly/{}.zip",
            zip_name
        );
        println!("cargo:warning=WebUI library not found. Downloading: {}", url);
        fetch_library(&url, &zip_name, lib_filename, &manifest_dir);
        println!("cargo:warning=WebUI library ready.");
    }

    println!("cargo:rustc-link-search=native={}", manifest_dir);
    println!("cargo:rustc-link-lib=static=webui-2-static");

    if target_os == "windows" {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=advapi32");
        println!("cargo:rustc-link-lib=ole32");
    }
}

fn select_zip(os: &str, arch: &str, env: &str) -> String {
    match os {
        "windows" => {
            let cc = if env == "msvc" { "msvc" } else { "gcc" };
            format!("webui-windows-{}-x64", cc)
        }
        "macos" => {
            let a = if arch == "aarch64" { "arm64" } else { "x64" };
            format!("webui-macos-clang-{}", a)
        }
        "linux" => {
            let a = match arch {
                "x86_64" => "x64",
                "aarch64" => "arm64",
                "arm" => "arm",
                _ => panic!("Unsupported Linux architecture: {}", arch),
            };
            format!("webui-linux-gcc-{}", a)
        }
        _ => panic!("Unsupported OS: {}", os),
    }
}

fn fetch_library(url: &str, zip_name: &str, lib_filename: &str, dest_dir: &str) {
    let zip_path = Path::new(dest_dir).join(format!("{}.zip", zip_name));
    let extract_dir = Path::new(dest_dir).join(zip_name);

    // Download
    if cfg!(windows) {
        run(Command::new("powershell").args([
            "-NoProfile",
            "-Command",
            &format!(
                "Invoke-WebRequest -Uri '{}' -OutFile '{}'",
                url,
                zip_path.display()
            ),
        ]));
    } else {
        run(Command::new("curl").args([
            "-fsSL",
            "-o",
            zip_path.to_str().unwrap(),
            url,
        ]));
    }

    // Extract
    fs::create_dir_all(&extract_dir).unwrap();
    if cfg!(windows) {
        run(Command::new("powershell").args([
            "-NoProfile",
            "-Command",
            &format!(
                "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                zip_path.display(),
                dest_dir
            ),
        ]));
    } else {
        run(Command::new("unzip").args([
            "-q",
            "-o",
            zip_path.to_str().unwrap(),
            "-d",
            dest_dir,
        ]));
    }

    // Copy static lib to project root
    let extracted_lib = extract_dir.join(lib_filename);
    let dest_lib = Path::new(dest_dir).join(lib_filename);
    fs::copy(&extracted_lib, &dest_lib).unwrap_or_else(|e| {
        panic!(
            "Failed to copy '{}' to project root: {}",
            extracted_lib.display(),
            e
        )
    });

    // Cleanup
    let _ = fs::remove_file(&zip_path);
    let _ = fs::remove_dir_all(&extract_dir);
}

fn run(cmd: &mut Command) {
    let status = cmd.status().expect("Failed to execute command");
    if !status.success() {
        panic!("Command failed with status {}: {:?}", status, cmd);
    }
}
