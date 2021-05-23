#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod telnet;
mod options;
use crate::telnet::login;
use crate::options::parse_commandline;

use ::telnet::TelnetEvent;
use std::time::Duration;
use std::thread;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use rocket::State;
use std::net::SocketAddr;

struct CommandSender<T> {
    tx: Mutex<Sender<T>>,
}

#[post("/device/<index>/setlevel/<level>?<time>")]
fn set_light_level(index: u8, level: u8, time: Option<u16>, command_sender: State<CommandSender<String>>) {
    let command = match time {
        None => format!("#OUTPUT,{},1,{}\n", index, level),
        Some(duration) => format!("#OUTPUT,{},1,{},{}\n", index, level, duration)
    };

    let tx: Sender<String> = command_sender.tx.lock().unwrap().clone();
    drop(command_sender);
    tx.send(command).unwrap();
}

fn main() {
    let args = parse_commandline();
    let bridge_address = SocketAddr::from((args.addr, args.port));
    let (tx, rx) = channel::<String>();
    thread::spawn(move|| {
        let mut telnet = login(bridge_address).unwrap();
        let d = Duration::from_millis(20);
        loop {
            if let Ok(TelnetEvent::Data(data)) = telnet.read_nonblocking() {
                for s in String::from_utf8_lossy(&data).split_whitespace().filter(|&s| s != "GNET>") {
                    // Prints individual output lines without the prompt, perfect for sending down
                    // a broadcast queue
                    // ...except for error output, which includes a space between the decimal and
                    // hex number. Perhaps lines starting with '0x' can be ignored, if the decimal
                    // and hex values are always equivalent.
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

    rocket::ignite()
        .mount("/", routes![set_light_level])
        .manage(CommandSender {tx: Mutex::new(tx)})
        .launch();
}
