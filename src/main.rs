use telnet::{Telnet, TelnetEvent};

fn main() {
    println!("Connecting...");
    let mut telnet = Telnet::connect(("10.0.0.73", 23), 256)
        .expect("Could not connect to Smart Bridge!");
    println!("Connected");

    let username = "lutron\n";
    let password = "integration\n";
    let command = "#output,3,1,15\n";
    let mut sent = false;
    loop {
        println!("Reading...");
        let event = telnet.read().expect("Read error");
        println!("Read data");
        match event {
            TelnetEvent::Data(buffer) => {
                println!("{:?}", String::from_utf8_lossy(&buffer));
                if String::from_utf8_lossy(&buffer).eq("login: ") {
                    telnet.write(username.as_bytes()).expect("Write error");
                } else if String::from_utf8_lossy(&buffer).eq("password: ") {
                    telnet.write(password.as_bytes()).expect("Write error");
                } else if sent == false {
                    telnet.write(command.as_bytes()).expect("Write error");
                    sent = true;
                }
            },
            _ => {}
        }
    }
}
