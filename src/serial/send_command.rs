use crate::protocol::char_to_bytes::char_to_bytes;
use serialport::SerialPort;
use std::io::Write;

pub fn send_command(
    port: &mut Box<dyn SerialPort>,
    command: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Process each character in the command and send it over the serial port
    for c in command.chars() {
        let obfuscated: [u8; 4] = char_to_bytes(c);

        // Send the bytes
        if let Err(e) = port.write_all(&obfuscated) {
            eprintln!("Error sending data: {}", e);
            continue;
        }
        port.flush().expect("Failed to flush");
    }

    // Send CR LF sequence in obfuscated format to end the command
    send_crlf_sequence(port);

    Ok(())
}

fn send_crlf_sequence(port: &mut Box<dyn SerialPort>) {
    let cr_lf: [char; 2] = ['\r', '\n'];
    for c in cr_lf {
        let obfuscated: [u8; 4] = char_to_bytes(c);

        if let Err(e) = port.write_all(&obfuscated) {
            eprintln!("Error sending data: {}", e);
            continue;
        }
        port.flush().expect("Failed to flush");
    }
}
