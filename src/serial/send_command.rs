use crate::protocol::char_to_bytes::char_to_bytes;
use crate::protocol::bytes_to_char::bytes_to_char;
use serialport::SerialPort;
use std::io::{Write, Read};
use std::time::Duration;

pub fn send_command(
    port: &mut Box<dyn SerialPort>,
    command: &str,
    timeout: Duration,
) -> Result<String, Box<dyn std::error::Error>> {
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

    // Read the response from the serial port
    let response: String = read_response(port, timeout)?;

    Ok(response)
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

fn read_response(port: &mut Box<dyn SerialPort>, timeout: Duration) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer: Vec<u8> = vec![0; 1024];
    let mut response: Vec<u8> = Vec::new();

    // Set the read timeout
    port.set_timeout(timeout)?;

    loop {
        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    response.extend_from_slice(&buffer[..bytes_read]);
                } else {
                    break;
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::TimedOut {
                    break; // Timeout reached, exit the loop
                } else {
                    return Err(Box::new(e));
                }
            }
        }
    }

    let decoded_response: String = bytes_to_char(&response);
    Ok(decoded_response)
}
