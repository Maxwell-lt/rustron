use std::net::IpAddr;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Control server for Lutron SmartBridge Pro", author)]
pub struct Commandline {
    #[structopt(help = "IP address of the SmartBridge Pro", short, long = "address")]
    pub addr: IpAddr,
    #[structopt(
        help = "Telnet port of the SmartBridge Pro",
        short,
        long,
        default_value = "23"
    )]
    pub port: u16,
    #[structopt(help = "Path to integration report", short = "r", long = "report")]
    pub integration_report_path: Option<PathBuf>,
}

pub fn parse_commandline() -> Commandline {
    Commandline::from_args()
}
