fn main() {
    cxx_build::bridge("src/testa.rs")
        .include("include")
        .file("src/testa.cc")
        .flag_if_supported("-std=c++14")
        .compile("testa");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/testa.cc");
}