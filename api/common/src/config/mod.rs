use std::env;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Database {
    url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Tushare {
    token: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Ms {
    pub email: String,
    pub password: String,
    pub login_url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AppConfig {
    database: Database,
    tushare: Tushare,
    ms: Ms,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE");
        let project_dir = env::var("PROJECT_DIR").expect("PROJECT_DIR is not set in .env file");

        let mut cfg_builder = Config::builder()
            .add_source(File::with_name(&format!("{project_dir}/common/src/config/files/default.toml")));
        if let Ok(run_mode) = run_mode {
            cfg_builder = cfg_builder.add_source(File::with_name(&format!("{project_dir}/common/src/config/files/{run_mode}.toml")).required(false));
        }
        let s = cfg_builder.add_source(File::with_name(&format!("{project_dir}/common/src/config/files/local.toml")).required(false)).build()?;
        s.try_deserialize()
    }

    pub fn database_url(&self) -> String {
        self.database.url.clone()
    }

    pub fn tushare_token(&self) -> String {
        self.tushare.token.clone()
    }

    pub fn mstar(&self) -> &Ms {
        &self.ms
    }
}