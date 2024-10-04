use std::path::PathBuf;
use reqwest;

fn main() {
    let url = "https://github.com/webui-dev/webui/raw/refs/tags/2.5.0-beta.2/include/webui.h";

    download(url);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("webui.h")
        .allowlist_type("webui.*")
        .allowlist_function("webui.*")
        .rustified_enum("webui.*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("bindgen.rs")
        .expect("Couldn't write bindings!");
}

fn download(url: &str) {
    let mut response = reqwest::blocking::get(url).unwrap();
    assert!(response.status().is_success());

    let mut dest = {
        let path = PathBuf::from("webui.h");
        std::fs::File::create(&path).unwrap()
    };
    std::io::copy(&mut response, &mut dest).unwrap();
}