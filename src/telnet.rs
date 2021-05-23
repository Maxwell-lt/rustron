use telnet::{Telnet, TelnetEvent};
use std::net::ToSocketAddrs;
use anyhow::Result;

pub fn login<A: ToSocketAddrs>(address: A) -> Result<Telnet> {
    const USERNAME: &str = "lutron\n";
    const PASSWORD: &str = "integration\n";

    let mut conn: Telnet = Telnet::connect(address, 65535)?;

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
                        // At prompt, fully logged in
                        break
                    },
                    _ => {
                        continue
                    }
                }
            }
            _ => continue
        }
    }
    Ok(conn)
}
