mod protocol {
    pub mod char_to_bytes;
    pub mod bytes_to_char;
}

mod serial {
    pub mod connect;
    pub mod serial_read;
    pub mod serial_write;
}

use serial::connect::connect;
use serial::serial_read::serial_read;
use serial::serial_write::serial_write;

use dotenv::dotenv;
use serialport::SerialPort;
use std::error::Error;

fn main() {
    dotenv().ok(); // Load environment variables from .env file

    // Connect to the serial port
    let port_name: String = std::env::var("SERIAL_PORT_NAME").expect("SERIAL_PORT environment variable not set");
    let baud_rate: u32 = std::env::var("SERIAL_BAUD_RATE")
        .unwrap_or_else(|_| "9600".to_string())
        .parse()
        .expect("Invalid baud rate");

    let port_result: Result<Box<dyn SerialPort + 'static>, Box<dyn Error + 'static>> = connect(&port_name, baud_rate);
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
