use serialport::SerialPort;
use std::io::{self, Write};
use crate::protocol::char_to_bytes::char_to_bytes;

pub fn serial_write(port: &mut Box<dyn SerialPort>) {
    println!("Enter characters to send (press Enter to send):");
    
    loop {
        // Read from terminal
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        // Process each character
        for c in input.trim().chars() {
            let obfuscated: [u8; 4] = char_to_bytes(c);
            println!("Sending '{}' as bytes:", c);
            for byte in obfuscated.iter() {
                println!("Byte: {:08b} (0x{:02X})", byte, byte);
            }
            
            // Send the bytes
            if let Err(e) = port.write_all(&obfuscated) {
                eprintln!("Error sending data: {}", e);
                continue;
            }
            port.flush().expect("Failed to flush");
        }
    }
} 