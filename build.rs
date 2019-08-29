extern crate cc;

//#[cfg(feature = "bindgen")]
extern crate bindgen;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

use std::fs::File;
use std::io::Write;
use std::path::Path;

// using clang-format to format c/c++ code
// C/C++/Java/JavaScript/Objective-C/Protobuf
// http://clang.llvm.org/docs/ClangFormat.html
fn run_clang_format(filename: &str) -> io::Result<bool> {
    let status = process::Command::new("clang-format")
        .args(&[
            "-style",
            "{BasedOnStyle: Google, ColumnLimit: 150, IndentWidth: 4,TabWidth: 4}",
        ])
        .args(&["-i", filename])
        .status()?;

    Ok(status.success())
}

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .header("./csrc/voxelizer/voxelizer.h")
        .header("./csrc/voxelizer/impl/voxelizer-impl.h")
        .generate_inline_functions(true)
        .generate()
        .expect("Unable to generate bindings.");

    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("src").join("bindings.rs"))
        .expect("Couldn't write bindings.");
}


fn main() {

    let files = vec![
        // recast
        "./csrc/voxelizer/impl/voxelizer.c",
    ];

    let headers = vec![
        // recast
        "./csrc/voxelizer/voxelizer.h",
    ];

    let bridges = vec!["./csrc/voxelizer/impl/voxelizer.c"];

    // for f in &files {
    //     run_clang_format(&f);
    // }
    // for f in &headers {
    //     run_clang_format(&f);
    // }
    for f in &bridges {
        run_clang_format(&f);
    }

    cc::Build::new()
        .cpp(true)
        .include("./csrc/voxelizer/impl")
        .include("./csrc/voxelizer")
        .files(&files)
        // MUST for slvs library !!!
        .define("LIBRARY", None)
        .flag_if_supported("-O2")
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wno-null-dereference")
        .flag_if_supported("-Wno-sign-compare")
        // .flag_if_supported("-Wno-write-strings")
        // .flag_if_supported("-Wno-unused-parameter")
        // .flag_if_supported("-fpermissive")
        .compile("libvoxelizer.o");

    generate_bindings();
}
