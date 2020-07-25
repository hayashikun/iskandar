use clap::Clap;
use std::str::FromStr;

#[derive(Clap)]
#[clap(version = "1.0", author = "R. Hayashi")]
pub struct Opts {
    #[clap(subcommand)]
    pub target: Target,
}

#[derive(Clap)]
pub enum Target {
    Init,
    Deploy(DeployOpts),
    Benchmark(BenchmarkOpts),
    Nginx(NginxOpts),
    Mysql(MysqlOpts),
}

#[derive(Clap)]
pub struct DeployOpts {
    #[clap(short, long, about = "Without git pull")]
    pub wo_pull: bool,

    #[clap(short, long, about = "Dry run")]
    pub dry: bool,
}

#[derive(Clap)]
pub struct BenchmarkOpts {
    #[clap(short, long, about = "Save nginx access log")]
    pub access_log: bool,

    #[clap(short, long, about = "Save mysql slow log")]
    pub slow_log: bool,

    #[clap(short, long, about = "Commit results")]
    pub commit: bool,
}

#[derive(Clap)]
pub struct NginxOpts {
    #[clap(subcommand)]
    pub action: NginxAction,

    #[clap(short, long, about = "Dry run")]
    pub dry: bool,
}

#[derive(Clap)]
pub enum NginxAction {
    #[clap(about = "Copy nginx_conf_file from nginx_conf_dir to project dir, and make backup")]
    Init,
    #[clap(about = "Reload nginx")]
    Reload,
    #[clap(about = "Copy nginx_conf_file from project dir to nginx_conf_dir")]
    Apply,
}

impl FromStr for NginxAction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "init" => Ok(NginxAction::Init),
            "reload" => Ok(NginxAction::Reload),
            "apply" => Ok(NginxAction::Apply),
            _ => Err("no match"),
        }
    }
}

#[derive(Clap)]
pub struct MysqlOpts {
    #[clap(subcommand)]
    pub action: MysqlAction,

    #[clap(short, long, about = "Dry run")]
    pub dry: bool,
}

#[derive(Clap)]
pub enum MysqlAction {
    #[clap(about = "Copy mysql_conf_file from mysql_conf_dir to project dir, and make backup")]
    Init,
    #[clap(about = "Restart mysql")]
    Restart,
    #[clap(about = "Copy mysql_conf_file from project dir to mysql_conf_dir")]
    Apply,
}

impl FromStr for MysqlAction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "restart" => Ok(MysqlAction::Restart),
            "backup" => Ok(MysqlAction::Init),
            "apply" => Ok(MysqlAction::Apply),
            _ => Err("no match"),
        }
    }
}
