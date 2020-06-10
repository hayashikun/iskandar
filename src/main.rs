use crate::config::Config;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::{env, process};

mod config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 2 {
        eprintln!("Not enough arguments");
        process::exit(1);
    }
    match args[1].as_str() {
        "init" => init(),
        "nginx" => nginx(args[2..].to_vec()),
        _ => {
            eprintln!("Invalid arguments");
            process::exit(1);
        }
    };
}

fn init() {
    let mut config = config::Config::template();
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

fn nginx(args: Vec<String>) {
    let config = load_config();
    println!("{:?}, {:?}", args, config);
    let project_root = PathBuf::from(config.project_root);
    let nginx_conf_dir = PathBuf::from(config.nginx_conf_dir);

    if args.len() < 1 {
        eprintln!("Not enough arguments");
        process::exit(1);
    }
    let mut cmd = match args[0].as_str() {
        "restart" => Command::new(config.nginx_restart_command),
        "backup" => Command::new(format!(
            "cp -r {} {}",
            nginx_conf_dir.to_str().unwrap(),
            project_root.join("nginx.backup").to_str().unwrap()
        )),
        "apply" => Command::new(format!(
            "cp {} {}",
            project_root.join(config.nginx_conf_file).to_str().unwrap(),
            nginx_conf_dir.to_str().unwrap()
        )),
        "unapply" => Command::new(format!(
            "rm {}",
            nginx_conf_dir
                .join(config.nginx_conf_file)
                .to_str()
                .unwrap(),
        )),
        _ => panic!("Invalid option"),
    };

    println!("{:?}", cmd)
}
