mod protocol {
    pub mod char_to_bytes;
    pub mod bytes_to_char;
}

mod serial {
    pub mod list_ports;
    pub mod connect;
    pub mod stream_input;
}

use protocol::char_to_bytes::char_to_bytes;
use protocol::bytes_to_char::bytes_to_char;
use serial::list_ports::list_ports;
use serial::connect::connect;
use serial::stream_input::stream_input;

use serialport::SerialPort;
use std::error::Error;

fn main() {
    println!("Hello, world!");

    let input_char: char = 'A'; // Example character

    let ascii: u8 = input_char as u8;

    println!("Character: '{}'", input_char);
    println!("Hex: 0x{:02X}", ascii);
    println!("Binary: {:08b}", ascii);

    let obfuscated: [u8; 4] = char_to_bytes(input_char);

    println!("Obfuscated: ");
    for byte in obfuscated.iter() {
        println!("{:08b}", byte);
    }

    println!("REVERSE ------------------");

    let deobfuscated: char = bytes_to_char(obfuscated);
    println!("Deobfuscated: '{}'", deobfuscated);


    // Serial connection
    list_ports();

    let port_result: Result<Box<dyn SerialPort + 'static>, Box<dyn Error + 'static>> = connect("/dev/tty.usbserial-1110", 9600);
    let mut port: Box<dyn SerialPort + 'static> = match port_result {
        Ok(port) => {
            println!("Port opened successfully");
            port
        }
        Err(e) => {
            println!("Error opening port: {}", e);
            return;
        }
    };

    stream_input(&mut port);
    
    
}
