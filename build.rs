fn main() {
    // Deaktiviere Windows Resource File Generation
    if std::env::var("TARGET").unwrap_or_default().contains("windows") {
        // Skip Windows Resource compilation
        std::env::set_var("TAURI_SKIP_EMBEDDED_RESOURCE", "1");
    }
    tauri_build::build()
}
