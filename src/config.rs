use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project_root: String,
    pub branch: String,
    pub nginx_conf_file: String,
    pub mysql_conf_file: String,
    pub nginx_conf_dir: String,
    pub mysql_conf_dir: String,
    pub deploy_commands: Vec<String>,
}

impl Config {
    pub fn template() -> Config {
        return Config {
            project_root: String::from(""),
            branch: String::from("master"),
            nginx_conf_file: String::from("nginx.conf"),
            mysql_conf_file: String::from("mysql.cnf"),
            nginx_conf_dir: String::from("/etc/nginx/conf.d/"),
            mysql_conf_dir: String::from("/etc/mysql/conf.d/"),
            deploy_commands: Vec::new(),
        };
    }

    pub fn from_toml(s: String) -> Config {
        toml::from_str(s.as_str()).expect("Failed to read toml.")
    }

    pub fn to_toml(&self) -> String {
        toml::to_string(self).expect("Failed to convert toml.")
    }
}
