extern crate bindgen;

fn main() {
    println!("cargo:rustc-link-lib=wiiuse");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path: std::path::PathBuf = std::env::var("OUT_DIR").unwrap().into();
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
