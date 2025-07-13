fn main() {
    println!("cargo::rustc-link-lib=foo");
    println!("cargo::rustc-link-search=.");
}