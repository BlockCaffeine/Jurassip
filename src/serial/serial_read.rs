use serialport::SerialPort;
use std::fs::OpenOptions;
use std::io::Write;
use crate::protocol::bytes_to_char::bytes_to_char;
use std::thread;
use std::sync::{Arc, Mutex};

pub fn serial_read(port: &mut Box<dyn SerialPort>) {
    let mut buffer: Vec<u8> = vec![0; 1024];
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("serial_log.txt")
        .expect("Failed to open log file");

    loop {
        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let decoded_data = bytes_to_char(&buffer[..bytes_read]);
                    writeln!(log_file, "{}", decoded_data).expect("Failed to write to log file");
                    println!("Decoded Data: {}", decoded_data);
                }
            }
            Err(_e) => {
                continue;
            }
        }
    }
}
