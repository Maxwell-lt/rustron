use telnet::{Telnet, TelnetEvent};
use anyhow::Result;
use std::io::{self, Write};
use std::time::Instant;

fn main() -> Result<()> {
    let mut telnet: Telnet;
    {
        println!("Connecting...");
        let start = Instant::now();
        telnet = login("10.0.0.73")?;
        let duration = start.elapsed();
        println!("Connected after {} seconds", duration.as_secs());
    }

    loop {
        print!("Enter a new light level for the switch with ID 3: ");
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        match buffer.trim().parse::<f64>() {
            Ok(new_level) => {
                println!("You entered {}", new_level);
                let command = format!("#OUTPUT,3,1,{}\n", buffer.trim());
                println!("Sending command <{}>...", command.trim());
                telnet.write(command.as_bytes())?;
                if let TelnetEvent::Data(response) = telnet.read()? {
                    let data = String::from_utf8_lossy(&response);
                    for line in data.lines() {
                        if line != "GNET> " {
                            println!("{}", line);
                        }
                    }
                }
            },
            Err(e) => {
                println!("Invalid input: {}", e);
            }
        }
    }
}

fn login(address: &str) -> Result<Telnet> {
    const USERNAME: &str = "lutron\n";
    const PASSWORD: &str = "integration\n";

    let mut conn: Telnet = Telnet::connect((address, 23), 256)?;

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
