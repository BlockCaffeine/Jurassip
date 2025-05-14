use crate::protocol::char_to_bytes::char_to_bytes;
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
    let mut decoded_response = String::new();
    let mut temp_char: u8 = 0;
    let mut bit_position = 0;

    // Set the read timeout
    port.set_timeout(timeout)?;

    loop {
        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    for &raw_byte in &buffer[..bytes_read] {
                        // Extract bits 2 and 5 from the raw byte
                        let bit_2 = (raw_byte >> 2) & 1;
                        let bit_5 = (raw_byte >> 5) & 1;

                        // Reconstruct the character by setting the appropriate bits
                        temp_char |= bit_2 << bit_position;
                        temp_char |= bit_5 << (bit_position + 1);

                        bit_position += 2;

                        // If we've reconstructed a full character (8 bits), append it
                        if bit_position >= 8 {
                            decoded_response.push(temp_char as char);
                            temp_char = 0;
                            bit_position = 0;
                        }
                    }
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

        // Check if the response ends with CR LF (\r\n)
        if decoded_response.ends_with("\r\n") {
            break;
        }
    }

    // Remove the trailing CR LF (\r\n) before returning
    if decoded_response.ends_with("\r\n") {
        decoded_response.truncate(decoded_response.len() - 2);
    }

    Ok(decoded_response)
}
