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

    let command = match opts.action {
        opts::NginxAction::Restart => config.nginx_restart_command,
        opts::NginxAction::Backup => format!(
            "cp -r {} {}",
            nginx_conf_dir.to_str().unwrap(),
            project_root.join("nginx.backup").to_str().unwrap()
        ),
        opts::NginxAction::Apply => format!(
            "cp {} {}",
            project_root.join(config.nginx_conf_file).to_str().unwrap(),
            nginx_conf_dir.to_str().unwrap()
        ),
        opts::NginxAction::Unapply => format!(
            "rm {}",
            nginx_conf_dir
                .join(config.nginx_conf_file)
                .to_str()
                .unwrap(),
        ),
    };

    if opts.dry {
        println!("Dry run: {:?}", command);
    } else {
        run_command(command);
    }
}

fn mysql(opts: opts::MysqlOpts) {
    let config = load_config();
    let project_root = PathBuf::from(config.project_root);
    let mysql_conf_dir = PathBuf::from(config.mysql_conf_dir);

    let command = match opts.action {
        opts::MysqlAction::Restart => config.mysql_restart_command,
        opts::MysqlAction::Backup => format!(
            "cp -r {} {}",
            mysql_conf_dir.to_str().unwrap(),
            project_root.join("mysql.backup").to_str().unwrap()
        ),
        opts::MysqlAction::Apply => format!(
            "cp {} {}",
            project_root.join(config.mysql_conf_file).to_str().unwrap(),
            mysql_conf_dir.to_str().unwrap()
        ),
        opts::MysqlAction::Unapply => format!(
            "rm {}",
            mysql_conf_dir
                .join(config.mysql_conf_file)
                .to_str()
                .unwrap(),
        ),
    };

    if opts.dry {
        println!("Dry run: {:?}", command);
    } else {
        run_command(command);
    }
}
