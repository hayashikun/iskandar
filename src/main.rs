use std::fs::File;
use std::io::Write;
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
