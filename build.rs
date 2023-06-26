fn main() {
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

    println!("cargo:rustc-link-search=native=libs");
}
