#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod integration;
mod options;
mod telnet;
use crate::telnet::TelnetConn;
use anyhow::Result;
use integration::IntegrationReport;
use rocket::State;
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct CommandSender<T> {
    tx: Mutex<Sender<T>>,
}

#[post("/device/<index>/setlevel/<level>?<time>")]
fn set_light_level(
    index: u8,
    level: u8,
    time: Option<u16>,
    command_sender: State<CommandSender<String>>,
) {
    let command = match time {
        None => format!("#OUTPUT,{},1,{}\n", index, level),
        Some(duration) => format!("#OUTPUT,{},1,{},{}\n", index, level, duration),
    };

    let tx: Sender<String> = command_sender.tx.lock().unwrap().clone();
    drop(command_sender);
    tx.send(command).unwrap();
}

fn main() -> Result<()> {
    let args: options::Commandline = options::parse_commandline();
    let bridge_address = SocketAddr::from((args.addr, args.port));
    let (tx, rx) = channel::<String>();
    thread::spawn(move || {
        let mut telnet: TelnetConn = TelnetConn::from_address(bridge_address).unwrap();
        let d = Duration::from_millis(20);
        loop {
            if let Ok(data) = telnet.read_nonblocking() {
                for s in data {
                    println!("{}", s);
                }
            }
            if let Ok(data) = rx.try_recv() {
                println!("{}", data);
                telnet.write(&data.into_bytes()).unwrap();
            }
            thread::sleep(d);
        }
    });

    if let Some(report_path) = args.integration_report_path {
        let report: IntegrationReport =
            serde_json::from_reader(BufReader::new(File::open(report_path)?))?;
        println!("Integration Report processed. Data:\n{:#?}", report);
    }

    rocket::ignite()
        .mount("/", routes![set_light_level])
        .manage(CommandSender { tx: Mutex::new(tx) })
        .launch();
    Ok(())
}
