pub mod formatted_strings;
pub mod types;

pub fn confy_proxy_load<'a, T>(file_name: &'a str) -> Result<T, confy::ConfyError>
where
    T: serde::de::DeserializeOwned + serde::Serialize + Default,
{
    let config_file = config_dir()?.join(build_file_name(file_name));
    let config = confy::load_path::<T>(config_file)?;
    Ok(config)
}

pub fn confy_proxy_store<'a, T>(file_name: &'a str, config: T) -> Result<(), confy::ConfyError>
where
    T: serde::de::DeserializeOwned + serde::Serialize + Default,
{
    let config_file = config_dir()?.join(build_file_name(file_name));
    confy::store_path(config_file, config)?;
    Ok(())
}

fn config_dir() -> Result<std::path::PathBuf, confy::ConfyError> {
    let current_exe = std::env::current_exe()
        .map_err(|e| confy::ConfyError::BadConfigDirectory(e.to_string()))?;
    let config_dir = current_exe
        .parent()
        .ok_or(confy::ConfyError::BadConfigDirectory(
            "No parent directory".to_string(),
        ))?
        .join("config");
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| confy::ConfyError::BadConfigDirectory(e.to_string()))?;
    }
    Ok(config_dir)
}
fn build_file_name(file_name: &str) -> String {
    format!("{}.toml", file_name)
}