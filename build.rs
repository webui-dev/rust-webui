fn main() {
    // WebUI static lib
    println!("cargo:rustc-link-lib=webui-2-static");

    #[cfg(target_os = "windows")]
    {
      println!("cargo:rustc-link-lib=user32");
      println!("cargo:rustc-link-lib=shell32");
    }
}
