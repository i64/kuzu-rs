const PATH: &str = "/home/barelylegal/git/clones/kuzu/";

fn build() {
    let dst = cmake::Config::new(PATH)
        .configure_arg(".")
        .very_verbose(true)
        .no_build_target(true)
        .cflag("-kuzu")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=kuzu");

    println!("cargo:rerun-if-changed={PATH}/*");
}
fn main() {
    // build();
    println!("cargo:rustc-link-lib=dylib=kuzu");
    println!("cargo:rustc-link-lib=dylib=pthread");

    println!("cargo:rustc-link-search=native=libs");
}
