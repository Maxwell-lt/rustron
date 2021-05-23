use structopt::StructOpt;
use std::net::IpAddr;

#[derive(Debug, StructOpt)]
#[structopt(about = "Control server for Lutron SmartBridge Pro")]
pub struct Commandline {
    #[structopt(help = "IP address of the SmartBridge Pro")]
    pub addr: IpAddr,
    #[structopt(help = "Telnet port of the SmartBridge Pro")]
    pub port: u16,
}

pub fn parse_commandline() -> Commandline {
    Commandline::from_args()
}
