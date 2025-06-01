fn main() {
    // Only link Windows-specific libraries when building for Windows
    #[cfg(windows)]
    {
        println!("cargo:rustc-link-lib=advapi32");
        println!("cargo:rustc-link-lib=wevtapi");
    }
    
    // Linux-specific build configuration
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=gtk-3");
        println!("cargo:rustc-link-lib=webkit2gtk-4.0");
        println!("cargo:rustc-link-lib=glib-2.0");
    }
    
    // macOS-specific build configuration
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=WebKit");
    }
}
