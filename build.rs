fn main() {
    println!("cargo::rustc-link-lib=foo");
    println!("cargo::rustc-link-search=.");
    println!("cargo::rerun-if-changed=libfoo.a"); // https://github.com/rust-lang/cargo/issues/12502
}