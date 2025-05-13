mod protocol {
    pub mod char_to_bytes;
    pub mod bytes_to_char;
    pub mod utils;
}

mod serial {
    pub mod list_ports;
    pub mod connect;
    pub mod serial_read;
    pub mod serial_write;
}

use protocol::char_to_bytes::char_to_bytes;
use protocol::bytes_to_char::bytes_to_char;
use protocol::utils::format_obfuscated_bytes;

use serial::list_ports::list_ports;
use serial::connect::connect;
use serial::serial_read::serial_read;
use serial::serial_write::serial_write;

use serialport::SerialPort;
use std::error::Error;

fn main() {
    // Example character to demonstrate conversion of a char to 4 obfuscated bytes and in reverse

    let input_char: char = 'N'; // Example character

    let ascii: u8 = input_char as u8;

    println!("Character: '{}'", input_char);
    println!("Hex: 0x{:02X}", ascii);
    println!("Binary: {:08b}", ascii);

    let obfuscated: [u8; 4] = char_to_bytes(input_char);

    println!("Obfuscated: ");
    for byte in obfuscated.iter() {
        println!("{}", format_obfuscated_bytes(*byte));
    }

    println!("REVERSE ------------------");

    let deobfuscated: char = bytes_to_char(obfuscated);
    println!("Deobfuscated: '{}'", deobfuscated);


    // Serial connection testing
    list_ports();

    // Connect to the serial port
    let port_result: Result<Box<dyn SerialPort + 'static>, Box<dyn Error + 'static>> = connect("/dev/tty.usbserial-140", 9600);
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

    serial_write(&mut port); // TESTING ONLY

    serial_read(&mut port); // TESTING ONLY - unreachable
    
}
