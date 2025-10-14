use std::{thread::sleep, time::Duration};
use telnet::{Event, Telnet};

fn main() {
    let mut conn = Telnet::connect(("pch.lan.id264.net", 23), 256).expect("Couldn't connect");

    // Wait for prompt (e.g., "$ " or "# ")
    loop {
        match conn.read_timeout(Duration::from_secs(5)) {
            Ok(Event::Data(buffer)) => {
                let text = String::from_utf8_lossy(&buffer);
                println!("RECV: {}", text);
                if text.trim_end().ends_with('#') || text.trim_end().ends_with('>') {
                    break;
                }
            }
            Ok(Event::TimedOut) => {
                eprintln!("Timed out waiting for prompt.");
                return;
            }
            Err(e) => {
                eprintln!("Error reading from telnet: {}", e);
                return;
            }
            _ => {}
        }
    }

    println!("Sending reboot command...");
    conn.write(b"/sbin/reboot\r\n")
        .expect("Failed to write command");

    // Wait a bit so it doesn't exit before command is processed
    sleep(Duration::from_secs(1));
}
