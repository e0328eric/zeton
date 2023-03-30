use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=/usr/X11R6/lib");

    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=Xft");
    println!("cargo:rustc-link-lib=fontconfig");

    let bindings = bindgen::Builder::default()
        .clang_arg("-I/usr/X11R6/include")
        .clang_arg("-I/usr/include/freetype2")
        .header("src/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
