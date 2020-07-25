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
    Deploy,
    Benchmark,
    Nginx(NginxOpts),
    Mysql(MysqlOpts),
}

#[derive(Clap)]
pub struct NginxOpts {
    pub action: NginxAction,

    #[clap(short, long, about = "Dry run")]
    pub dry: bool,
}

pub enum NginxAction {
    Init,
    Reload,
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
    pub action: MysqlAction,

    #[clap(short, long, about = "Dry run")]
    pub dry: bool,
}

pub enum MysqlAction {
    Restart,
    Init,
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
