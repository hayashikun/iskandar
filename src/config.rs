use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub dry_run: bool,
    pub project_root: String,
    pub nginx_conf_file: String,
    pub mysql_conf_file: String,
    pub nginx_conf_dir: String,
    pub mysql_conf_dir: String,
    pub nginx_restart_command: String,
    pub mysql_restart_command: String,
    pub benchmark_command: String,
    pub benchmark_score_regex: String,
}

impl Config {
    pub fn template() -> Config {
        return Config {
            dry_run: false,
            project_root: "".to_string(),
            nginx_conf_file: "nginx.conf".to_string(),
            mysql_conf_file: "mysql.cnf".to_string(),
            nginx_conf_dir: "/etc/nginx/conf.d/".to_string(),
            mysql_conf_dir: "/etc/mysql/conf.d/".to_string(),
            nginx_restart_command: "echo restart nginx".to_string(),
            mysql_restart_command: "echo restart mysql".to_string(),
            benchmark_command:
                "echo benchmark start; echo benchmark score: 884 point; echo benchmark end"
                    .to_string(),
            benchmark_score_regex: r"score: (\d+) point".to_string(),
        };
    }

    pub fn from_toml(s: String) -> Config {
        toml::from_str(s.as_str()).expect("Failed to read toml.")
    }

    pub fn to_toml(&self) -> String {
        toml::to_string(self).expect("Failed to convert toml.")
    }
}
