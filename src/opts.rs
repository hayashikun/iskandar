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
    Redis(RedisOpts),
}

#[derive(Clap)]
pub struct NginxOpts {
    pub action: NginxAction,

    #[clap(short, long, about = "Dry run")]
    pub dry: bool,
}

pub enum NginxAction {
    Restart,
    Backup,
    Apply,
    Unapply,
}

impl FromStr for NginxAction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "restart" => Ok(NginxAction::Restart),
            "backup" => Ok(NginxAction::Backup),
            "apply" => Ok(NginxAction::Apply),
            "unapply" => Ok(NginxAction::Unapply),
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
    Backup,
    Apply,
    Unapply,
}

impl FromStr for MysqlAction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "restart" => Ok(MysqlAction::Restart),
            "backup" => Ok(MysqlAction::Backup),
            "apply" => Ok(MysqlAction::Apply),
            "unapply" => Ok(MysqlAction::Unapply),
            _ => Err("no match"),
        }
    }
}

#[derive(Clap)]
pub struct RedisOpts {
    pub action: RedisAction,

    #[clap(short, long, about = "Dry run")]
    pub dry: bool,
}

pub enum RedisAction {
    Restart,
    Backup,
    Apply,
    Unapply,
}

impl FromStr for RedisAction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "restart" => Ok(RedisAction::Restart),
            "backup" => Ok(RedisAction::Backup),
            "apply" => Ok(RedisAction::Apply),
            "unapply" => Ok(RedisAction::Unapply),
            _ => Err("no match"),
        }
    }
}
