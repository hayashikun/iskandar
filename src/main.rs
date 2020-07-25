use crate::opts::DeployOpts;
use chrono::{DateTime, Local};
use clap::Clap;
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

mod config;
mod opts;

fn main() {
    let opts: opts::Opts = opts::Opts::parse();

    match opts.target {
        opts::Target::Init => init(),
        opts::Target::Deploy(opts) => deploy(opts),
        opts::Target::Benchmark => benchmark(),
        opts::Target::Nginx(opts) => nginx(opts),
        opts::Target::Mysql(opts) => mysql(opts),
    };
}

fn init() {
    let config = load_config();
    fs::write("iskandar.toml", config.to_toml()).expect("Failed to save iskandar.toml");
}

fn load_config() -> config::Config {
    let toml = fs::read_to_string("iskandar.toml").unwrap_or("".to_string());
    config::Config::from_toml(toml)
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

fn deploy(opts: DeployOpts) {
    let config = load_config();
    let mut command = config.deploy_command;

    if opts.wo_pull {
        println!("Deploy without git pull");
    } else {
        command = format!("git pull origin {}; {}", config.git_branch, command);
    }

    if opts.dry {
        println!("Dry run: {:?}", command);
    } else {
        run_command(command);
    }
}

fn save_score(score: f32) {
    let datetime: DateTime<Local> = Local::now();
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("score.csv")
        .unwrap();
    if file.metadata().unwrap().len() == 0 {
        writeln!(file, "datetime, score").unwrap();
    }
    writeln!(file, "{}, {}", datetime, score).unwrap();
}

fn benchmark() {
    let config = load_config();
    let lines = run_command(config.benchmark_command);

    let re = Regex::new(&config.benchmark_score_regex).unwrap();
    for line in lines.iter().rev() {
        if let Some(caps) = re.captures(&line) {
            let score: f32 = caps
                .get(1)
                .expect("Failed to find score")
                .as_str()
                .trim()
                .parse()
                .expect("Failed to parse score");
            save_score(score);
            break;
        }
    }
}

fn nginx(opts: opts::NginxOpts) {
    let config = load_config();
    let project_root = PathBuf::from(config.project_root);
    let nginx_conf_dir = PathBuf::from(config.nginx_conf_dir);
    let nginx_conf_file = config.nginx_conf_file;

    let command = match opts.action {
        opts::NginxAction::Reload => config.nginx_reload_command,
        opts::NginxAction::Init => format!(
            "cp {} {}; cp {} {}",
            nginx_conf_dir.join(&nginx_conf_file).to_str().unwrap(),
            project_root.join(&nginx_conf_file).to_str().unwrap(),
            project_root.join(&nginx_conf_file).to_str().unwrap(),
            project_root
                .join(format!("{}.backup", &nginx_conf_file))
                .to_str()
                .unwrap(),
        ),
        opts::NginxAction::Apply => format!(
            "cp {} {}",
            project_root.join(&nginx_conf_file).to_str().unwrap(),
            nginx_conf_dir.join(&nginx_conf_file).to_str().unwrap()
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
    let mysql_conf_file = config.mysql_conf_file;

    let command = match opts.action {
        opts::MysqlAction::Restart => config.mysql_restart_command,
        opts::MysqlAction::Init => format!(
            "cp {} {}; cp {} {}",
            mysql_conf_dir.join(&mysql_conf_file).to_str().unwrap(),
            project_root.join(&mysql_conf_file).to_str().unwrap(),
            project_root.join(&mysql_conf_file).to_str().unwrap(),
            project_root
                .join(format!("{}.backup", &mysql_conf_file))
                .to_str()
                .unwrap(),
        ),
        opts::MysqlAction::Apply => format!(
            "cp {} {}",
            project_root.join(&mysql_conf_file).to_str().unwrap(),
            mysql_conf_dir.to_str().unwrap()
        ),
    };

    if opts.dry {
        println!("Dry run: {:?}", command);
    } else {
        run_command(command);
    }
}
