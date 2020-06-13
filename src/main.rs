use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use clap::Clap;

use crate::config::Config;

mod config;
mod opts;

fn main() {
    let opts: opts::Opts = opts::Opts::parse();

    match opts.target {
        opts::Target::Init => init(),
        opts::Target::Benchmark => benchmark(),
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

fn run_command(command: String) -> Vec<String> {
    let child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run command");
    let reader = BufReader::new(child.stdout.unwrap());
    let mut results = Vec::new();
    for l in reader.lines() {
        let line = l.unwrap();
        println!("{}", line);
        results.push(line);
    }
    return results;
}

fn benchmark() {
    let config = load_config();
    run_command(config.benchmark_command);
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
        cmd.stdout(Stdio::piped())
            .spawn()
            .expect("Failed to run command");
    }
}

fn mysql(opts: opts::MysqlOpts) {
    let config = load_config();
    let project_root = PathBuf::from(config.project_root);
    let mysql_conf_dir = PathBuf::from(config.mysql_conf_dir);

    let mut cmd = match opts.action {
        opts::MysqlAction::Restart => Command::new(config.mysql_restart_command),
        opts::MysqlAction::Backup => Command::new(format!(
            "cp -r {} {}",
            mysql_conf_dir.to_str().unwrap(),
            project_root.join("mysql.backup").to_str().unwrap()
        )),
        opts::MysqlAction::Apply => Command::new(format!(
            "cp {} {}",
            project_root.join(config.mysql_conf_file).to_str().unwrap(),
            mysql_conf_dir.to_str().unwrap()
        )),
        opts::MysqlAction::Unapply => Command::new(format!(
            "rm {}",
            mysql_conf_dir
                .join(config.mysql_conf_file)
                .to_str()
                .unwrap(),
        )),
    };

    if opts.dry {
        println!("{:?}", cmd);
    } else {
        cmd.stdout(Stdio::piped())
            .spawn()
            .expect("Failed to run command");
    }
}
