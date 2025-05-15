use crate::protocol::char_to_bytes::char_to_bytes;
use serialport::SerialPort;
use std::io::Write;
use anyhow::Result;

pub fn send_command(
    port: &mut Box<dyn SerialPort>,
    command: &str,
) -> Result<()> {
    for c in command.chars() {
        let obfuscated: [u8; 4] = char_to_bytes(c);

        if let Err(e) = port.write_all(&obfuscated) {
            eprintln!("Error sending data: {}", e);
            continue;
        }
        port.flush()?;
    }

    send_crlf_sequence(port)?;
    Ok(())
}

fn send_crlf_sequence(port: &mut Box<dyn SerialPort>) -> Result<()> {
    let cr_lf: [char; 2] = ['\r', '\n'];
    for c in cr_lf {
        let obfuscated: [u8; 4] = char_to_bytes(c);

        if let Err(e) = port.write_all(&obfuscated) {
            eprintln!("Error sending data: {}", e);
            continue;
        }
        port.flush()?;
    }
    Ok(())
}
