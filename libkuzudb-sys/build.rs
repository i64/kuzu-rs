use std::{path::Path, process::Command};

const CMAKE_NAME: &str = "CMakeLists.txt";
const TOOLS_NAME: &str = "tools";
const SIMPLIFIED_CMAKE_NAME: &str = "simplified_cmake.txt";

fn link(kuzu_path: &Path, profile: &str) {
    let binding = std::fs::canonicalize(kuzu_path).unwrap();
    let kuzu_display_path = binding.display();

    println!("cargo:rustc-link-lib=static=kuzu");
    println!("cargo:rustc-link-lib=static=antlr4_cypher");
    println!("cargo:rustc-link-lib=static=antlr4_runtime");
    println!("cargo:rustc-link-lib=static=re2");
    println!("cargo:rustc-link-lib=static=utf8proc");
    println!("cargo:rustc-link-lib=static=parquet");
    println!("cargo:rustc-link-lib=static=arrow");
    println!("cargo:rustc-link-lib=static=arrow_bundled_dependencies");

    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=pthread");

    println!("cargo:rustc-link-search=native={kuzu_display_path}/build/{profile}/src/");

    println!("cargo:rustc-link-search=native={kuzu_display_path}/build/{profile}/third_party/antlr4_cypher/");
    println!("cargo:rustc-link-search=native={kuzu_display_path}/build/{profile}/third_party/antlr4_runtime/");

    println!(
        "cargo:rustc-link-search=native={kuzu_display_path}/external/build/arrow/install/lib/"
    );

    println!(
        "cargo:rustc-link-search=native={kuzu_display_path}/build/{profile}/third_party/utf8proc/"
    );
    println!("cargo:rustc-link-search=native={kuzu_display_path}/build/{profile}/third_party/re2/");
}

fn overwrite_cmake(kuzu_path: &Path) {
    let tools_path = kuzu_path.join(TOOLS_NAME);
    let cmake_path = kuzu_path.join(CMAKE_NAME);
    let simplified = Path::new(SIMPLIFIED_CMAKE_NAME);

    assert!(cmake_path.exists());
    assert!(simplified.exists());
    let _ = std::fs::remove_file(&cmake_path);
    let _ = std::fs::remove_dir(tools_path);

    assert!(std::fs::copy(simplified, &cmake_path).is_ok())
}

fn run_make(kuzu_path: &Path, profile: &str) {
    let current_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(kuzu_path).unwrap();

    let mut command = Command::new("make").arg(profile).spawn().unwrap();
    let _command_result = command.wait().unwrap();
    assert!(_command_result.success());

    std::env::set_current_dir(current_dir).unwrap();
}

fn main() {
    let kuzu_path = Path::new("./kuzu/");
    let profile = "release"; // get_profile();

    println!("cargo:rerun-if-changed=build.rs");

    assert!(kuzu_path.exists());
    assert!(kuzu_path.is_dir());

    overwrite_cmake(kuzu_path);
    run_make(kuzu_path, profile);
    link(kuzu_path, profile);
}
