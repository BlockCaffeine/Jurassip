use crate::protocol::char_to_bytes::char_to_bytes;
use crate::protocol::bytes_to_char::bytes_to_char;
use serialport::SerialPort;
use std::io::{Write, Read};

pub fn send_command(
    port: &mut Box<dyn SerialPort>,
    command: &str,
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
    let response: String = read_response(port)?;

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

fn read_response(port: &mut Box<dyn SerialPort>) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer: Vec<u8> = vec![0; 4096]; // Increase buffer size for larger responses
    let mut total_bytes_read = 0;

    loop {
        match port.read(&mut buffer[total_bytes_read..]) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break; // No more data to read
                }
                total_bytes_read += bytes_read;

                // Stop reading if buffer is full
                if total_bytes_read == buffer.len() {
                    break;
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                break; // Timeout reached, stop reading
            }
            Err(e) => {
                return Err(Box::new(e)); // Return any other errors
            }
        }
    }

    // Decode the response
    let response = bytes_to_char(&buffer[..total_bytes_read]);

    Ok(response)
}
