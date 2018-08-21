use std::path::{Path, PathBuf};

use base64;

use super::{cache, errors::Result, oauth, orm, queue, storage};

#[cfg(not(debug_assertions))]
pub fn version() -> String {
    format!("{}({})", env!("GIT_HEAD"), env!("BUILD_TIME"))
}

#[cfg(debug_assertions)]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
pub const HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
pub const BANNER: &'static str = r#"
_      ____ _______ _    _  _____
| |    / __ \__   __| |  | |/ ____|
| |   | |  | | | |  | |  | | (___
| |   | |  | | | |  | |  | |\___ \
| |___| |__| | | |  | |__| |____) |
|______\____/  |_|   \____/|_____/

"#;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub name: String,
    pub env: String,
    #[serde(rename = "secretkey")]
    pub secret_key: String, // 32-bits base64 encode string
    pub http: Http,
    pub oauth: oauth::Config,
    pub database: orm::Config,
    pub cache: cache::Config,
    pub queue: queue::Config,
    pub storage: storage::Config,
    pub elasticsearch: ElasticSearch,
    pub aws: Aws,
}

impl Config {
    pub fn secret_key(&self) -> Result<Vec<u8>> {
        let buf = base64::decode(&self.secret_key)?;
        Ok(buf)
    }
    pub fn is_production(&self) -> bool {
        self.env == "production"
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Http {
    pub port: u16,
    pub theme: String,
}

impl Http {
    pub fn views(&self) -> PathBuf {
        self.root().join("views")
    }
    pub fn assets(&self) -> PathBuf {
        self.root().join("assets")
    }

    fn root(&self) -> PathBuf {
        Path::new("themes").join(&self.theme)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Aws {
    #[serde(rename = "accesskeyid")]
    pub access_key_id: String,
    #[serde(rename = "secretaccesskey")]
    pub secret_access_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ElasticSearch {
    pub hosts: Vec<String>,
}
