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
    let mut buffer: Vec<u8> = vec![0; 1]; // Read one byte at a time
    let mut decoded_response = String::new();
    let mut in_bits = String::new();
    let start_time = std::time::Instant::now();

    // Set the read timeout
    port.set_timeout(timeout)?;

    loop {
        // Check for timeout
        if start_time.elapsed() > timeout {
            eprintln!("Timeout reading result");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Timeout reading result",
            )));
        }

        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let raw_byte = buffer[0];
                    let raw_bits = format!("{:08b}", raw_byte); // Convert byte to binary string

                    // Extract bits 2 and 5
                    in_bits.push(raw_bits.chars().nth(2).unwrap());
                    in_bits.push(raw_bits.chars().nth(5).unwrap());

                    // If we've reconstructed a full character (8 bits), decode it
                    if in_bits.len() == 8 {
                        let byte = u8::from_str_radix(&in_bits, 2).unwrap();
                        decoded_response.push(byte as char);
                        in_bits.clear();
                    }

                    // Check if the response ends with CR LF (\r\n)
                    if decoded_response.ends_with("\r\n") {
                        break;
                    }
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::TimedOut {
                    continue; // Retry on timeout
                } else {
                    return Err(Box::new(e));
                }
            }
        }
    }

    // Remove the trailing CR LF (\r\n) before returning
    if decoded_response.ends_with("\r\n") {
        decoded_response.truncate(decoded_response.len() - 2);
    }

    Ok(decoded_response)
}

