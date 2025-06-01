fn main() {
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=wevtapi");
}
