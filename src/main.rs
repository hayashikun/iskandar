use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::{env, process};

use clap::Clap;

use crate::config::Config;

mod config;
mod opts;

fn main() {
    let opts: opts::Opts = opts::Opts::parse();

    match opts.target {
        opts::Target::Init => init(),
        opts::Target::Nginx(opts) => nginx(opts),
        opts::Target::Mysql(opts) => mysql(opts),
    };
}

fn init() {
    let mut config = Config::template();
    config.project_root = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let mut file = File::create("iskandar.toml").unwrap();
    writeln!(&mut file, "{}", config.to_toml()).unwrap();
}

fn load_config() -> Config {
    let mut toml = String::new();
    File::open("iskandar.toml")
        .unwrap()
        .read_to_string(&mut toml)
        .unwrap();
    Config::from_toml(toml)
}

fn nginx(opts: opts::NginxOpts) {
    let config = load_config();
    let project_root = PathBuf::from(config.project_root);
    let nginx_conf_dir = PathBuf::from(config.nginx_conf_dir);

    let mut cmd = match opts.action {
        opts::NginxAction::Restart => Command::new(config.nginx_restart_command),
        opts::NginxAction::Backup => Command::new(format!(
            "cp -r {} {}",
            nginx_conf_dir.to_str().unwrap(),
            project_root.join("nginx.backup").to_str().unwrap()
        )),
        opts::NginxAction::Apply => Command::new(format!(
            "cp {} {}",
            project_root.join(config.nginx_conf_file).to_str().unwrap(),
            nginx_conf_dir.to_str().unwrap()
        )),
        opts::NginxAction::Unapply => Command::new(format!(
            "rm {}",
            nginx_conf_dir
                .join(config.nginx_conf_file)
                .to_str()
                .unwrap(),
        )),
    };

    if opts.dry {
        println!("{:?}", cmd);
    } else {
        cmd.spawn().expect("Failed to ")
    }
}

fn mysql(opts: opts::MysqlOpts) {}
