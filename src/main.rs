#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use telnet::{Telnet, TelnetEvent};
use anyhow::Result;
use std::time::Duration;
use std::thread;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use rocket::State;

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
    const ADDRESS: &str = "10.0.0.73";

    let (tx, rx) = channel::<String>();
    thread::spawn(move|| {
        let mut telnet = login(ADDRESS).unwrap();
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

fn login(address: &str) -> Result<Telnet> {
    const USERNAME: &str = "lutron\n";
    const PASSWORD: &str = "integration\n";

    let mut conn: Telnet = Telnet::connect((address, 23), 65535)?;

    loop {
        let evt = conn.read()?;
        match evt {
            TelnetEvent::Data(buf) => {
                let line = String::from_utf8_lossy(&buf);
                match line.to_string().as_str() {
                    "login: " => {
                        conn.write(USERNAME.as_bytes())?;
                        continue
                    },
                    "password: " => {
                        conn.write(PASSWORD.as_bytes())?;
                        continue
                    },
                    "GNET> " => {
                        return Ok(conn);
                    },
                    _ => {
                        continue
                    }
                }
            }
            _ => continue
        }
    }
}
