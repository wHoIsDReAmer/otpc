use std::fs::{self, File, OpenOptions, Permissions};
use std::io::{self, Read as _, Seek as _, SeekFrom, Write as _};
use std::os::unix::fs::PermissionsExt as _;
use std::path::PathBuf;
use std::env;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use anyhow::Result;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub secret: String,
    pub issuer: String,
}

fn load_config() -> Result<Config> {
    let home_dir = match env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
        Ok(path) => PathBuf::from(path),
        Err(_) => return Err(anyhow::anyhow!("Cannot found user directory")),
    };
    
    let config_path = home_dir.join(".otpc").join("config.toml");
    fs::create_dir_all(home_dir.join(".otpc"))?;

    let mut config_file = match File::open(&config_path) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                let mut new_config_file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .read(true)
                    .open(&config_path)?;
                new_config_file.set_permissions(Permissions::from_mode(0o600))?;

                let new_config = Config { accounts: vec![] };
                let new_config_str = toml::to_string(&new_config).map_err(|e| anyhow::anyhow!(e))?;
                new_config_file.write_all(new_config_str.as_bytes())?;

                new_config_file.seek(SeekFrom::Start(0))?;
                new_config_file
            } else {
                return Err(anyhow::anyhow!(e));
            }
        }
    };

    let mut config_str = String::new();
    config_file.read_to_string(&mut config_str)?;

    let config: Config = toml::from_str(&config_str).map_err(|e| anyhow::anyhow!(e))?;
    Ok(config)
}

pub(crate) fn get_config() -> Result<&'static Config> {
    static CONFIG: OnceLock<Config> = OnceLock::new();

    Ok(CONFIG.get_or_init(|| match load_config() {
        Ok(config) => config,
        Err(e) => panic!("Failed to load config: {}", e),
    }))
}
