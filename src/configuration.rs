use config::ConfigError;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub network: String,
    pub mint_accounts: Vec<String>,
}

pub fn setup_config() -> Result<Settings, ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");
    settings.merge(config::File::from(configuration_directory.join("settings")).required(true))?;
    settings.try_into()
}
