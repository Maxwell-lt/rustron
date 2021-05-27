use anyhow::{Context, Result};
use std::net::ToSocketAddrs;
use telnet::{Telnet, TelnetEvent};

const USERNAME: &str = "lutron\n";
const PASSWORD: &str = "integration\n";

pub struct TelnetConn {
    conn: Telnet,
}

impl TelnetConn {
    pub fn from_address<A: ToSocketAddrs>(address: A) -> Result<TelnetConn> {
        let mut conn: Telnet = Telnet::connect(address, 65535)?;

        loop {
            let evt = conn.read()?;
            match evt {
                TelnetEvent::Data(buf) => {
                    let line = String::from_utf8_lossy(&buf);

                    match line.to_string().as_str() {
                        "login: " => {
                            conn.write(USERNAME.as_bytes())?;
                            continue;
                        }
                        "password: " => {
                            conn.write(PASSWORD.as_bytes())?;
                            continue;
                        }
                        "GNET> " => {
                            // At prompt, fully logged in
                            break;
                        }
                        _ => continue,
                    }
                }
                _ => continue,
            }
        }
        Ok(TelnetConn { conn })
    }

    pub fn read_nonblocking(&mut self) -> Result<Vec<String>> {
        Ok(event_to_strings(
            self.conn
                .read_nonblocking()
                .context("Failed to read from telnet connection")?,
        ))
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        self.conn
            .write(data)
            .context("Failed to write to telnet connection")
    }
}

fn event_to_strings(evt: TelnetEvent) -> Vec<String> {
    if let TelnetEvent::Data(data) = evt {
        let string_data = String::from_utf8_lossy(&data);
        let mut lines = string_data.split_whitespace().collect::<Vec<_>>();
        // Add dummy value to end so that each value will be in the first slot of a window
        lines.push("");
        // Windows are used here so that error outputs can be merged after being split due to
        // whitespace. Error output looks like: ~ERROR,Enum(1, 0x00000001), hence checking if the
        // first character of a line is a 0 should be sufficient to identify it as the second half
        // of a split error message.
        // This could be improved by not splitting the error messages in the first place.
        lines
            .as_slice()
            .windows(2)
            .filter_map(|win| {
                if win[0] == "GNET>" {
                    None
                } else if let Some('0') = win[0].chars().next() {
                    None
                } else if let Some('0') = win[1].chars().next() {
                    Some(format!("{} {}", win[0], win[1]))
                } else {
                    Some(win[0].to_string())
                }
            })
            .collect()
    } else {
        vec![]
    }
}
