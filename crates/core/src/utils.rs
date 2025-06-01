use anyhow::Result;
use std::path::Path;

pub fn get_project_root() -> Result<String> {
    let current_dir = std::env::current_dir()?;
    let root = current_dir
        .ancestors()
        .find(|p| p.join("Cargo.toml").exists())
        .ok_or_else(|| anyhow::anyhow!("Could not find project root"))?;
    
    Ok(root.display().to_string())
}

pub fn get_config_path() -> Result<String> {
    let root = get_project_root()?;
    Ok(format!("{}{}config", root, std::path::MAIN_SEPARATOR))
}
