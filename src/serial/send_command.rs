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
    
    let bytes_read: usize = port.read(buffer.as_mut_slice())?;
    if bytes_read == 0 {
        return Ok("No response received".to_string());
    }
    let response_bytes: &[u8] = &buffer[..bytes_read];
    let response: String = bytes_to_char(response_bytes);

    Ok(response)
}
