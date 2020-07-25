use serde::{Deserialize, Serialize};
use std::env;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_project_root")]
    pub project_root: String,

    #[serde(default = "default_git_branch")]
    pub git_branch: String,

    #[serde(default = "default_nginx_conf_file")]
    pub nginx_conf_file: String,

    #[serde(default = "default_mysql_conf_file")]
    pub mysql_conf_file: String,

    #[serde(default = "default_nginx_conf_dir")]
    pub nginx_conf_dir: String,

    #[serde(default = "default_mysql_conf_dir")]
    pub mysql_conf_dir: String,

    #[serde(default = "default_nginx_reload_command")]
    pub nginx_reload_command: String,

    #[serde(default = "default_mysql_restart_command")]
    pub mysql_restart_command: String,

    #[serde(default = "default_benchmark_command")]
    pub benchmark_command: String,

    #[serde(default = "default_benchmark_score_regex")]
    pub benchmark_score_regex: String,

    #[serde(default = "default_nginx_access_log")]
    pub nginx_access_log: String,

    #[serde(default = "default_mysql_slow_log")]
    pub mysql_slow_log: String,

    #[serde(default = "default_deploy_command")]
    pub deploy_command: String,
}

fn default_project_root() -> String {
    env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}

fn default_git_branch() -> String {
    "master".to_string()
}

fn default_nginx_conf_file() -> String {
    "nginx.conf".to_string()
}

fn default_mysql_conf_file() -> String {
    "mysql.cnf".to_string()
}

fn default_nginx_conf_dir() -> String {
    "/etc/nginx/".to_string()
}

fn default_mysql_conf_dir() -> String {
    "/etc/mysql/conf.d/".to_string()
}

fn default_nginx_reload_command() -> String {
    "nginx -s reload".to_string()
}

fn default_mysql_restart_command() -> String {
    "echo please edit iskandar.toml to restart mysql".to_string()
}

fn default_benchmark_command() -> String {
    "echo benchmark start; echo benchmark score: 88.4 point; echo benchmark end".to_string()
}

fn default_benchmark_score_regex() -> String {
    r"score: ([\d.]+) point".to_string()
}

fn default_nginx_access_log() -> String {
    "/var/log/nginx/access.log".to_string()
}

fn default_mysql_slow_log() -> String {
    "/var/log/slow.log".to_string()
}

fn default_deploy_command() -> String {
    "echo please edit iskandar.toml; echo deploy!".to_string()
}

impl Config {
    pub fn from_toml(s: String) -> Config {
        toml::from_str(s.as_str()).expect("Failed to read toml.")
    }

    pub fn to_toml(&self) -> String {
        toml::to_string(self).expect("Failed to convert toml.")
    }
}
