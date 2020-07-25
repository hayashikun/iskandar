use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project_root: String,
    pub nginx_conf_file: String,
    pub mysql_conf_file: String,
    pub nginx_conf_dir: String,
    pub mysql_conf_dir: String,
    pub nginx_reload_command: String,
    pub mysql_restart_command: String,
    pub benchmark_command: String,
    pub benchmark_score_regex: String,
    pub deploy_command: String,
}

impl Config {
    pub fn template() -> Config {
        return Config {
            project_root: "".to_string(),
            nginx_conf_file: "nginx.conf".to_string(),
            mysql_conf_file: "mysql.cnf".to_string(),
            nginx_conf_dir: "/etc/nginx/".to_string(),
            mysql_conf_dir: "/etc/mysql/conf.d/".to_string(),
            nginx_reload_command: "nginx -s reload".to_string(),
            mysql_restart_command: "echo please edit iskandar.toml to restart mysql".to_string(),
            benchmark_command:
            "echo please edit iskandar.toml; echo benchmark start; echo benchmark score: 88.4 point; echo benchmark end"
                .to_string(),
            benchmark_score_regex: r"score: ([\d.]+) point".to_string(),
            deploy_command: "echo please edit iskandar.toml; echo deploy!".to_string(),
        };
    }

    pub fn from_toml(s: String) -> Config {
        toml::from_str(s.as_str()).expect("Failed to read toml.")
    }

    pub fn to_toml(&self) -> String {
        toml::to_string(self).expect("Failed to convert toml.")
    }
}
