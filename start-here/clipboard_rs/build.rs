// extern crate bindgen;

// use std::env;
// use std::path::PathBuf;

fn main() {
    // // Tell cargo to invalidate the built crate whenever the header changes

    // println!("cargo:rerun-if-changed=../ErrorRecord.hpp");

    // // The bindgen::Builder is the main entry point
    // // to bindgen, and lets you build up options for
    // // the resulting bindings.
    // let bindings = bindgen::Builder::default()
    //     //The input header we would like to generate bindings for
    //     .header("../ErrorRecord.hpp")
    //     //whitelist the stuff we care about
    //     .whitelist_type("ErrorRecord")
    //     .whitelist_function("ReportError")
    //     //Tell cargo to invalidate the built crate whenever any of the included header files change
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     //Finish the builder and generate the bindings
    //     .generate()
    //     // panic on failure
    //     .expect("Unable to generate bindings");

    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("generated_bindings.rs"))
    //     .expect("Couldn't write bindings!");
}
