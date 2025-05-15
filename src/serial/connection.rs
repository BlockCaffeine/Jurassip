use std::time::Duration;
use serialport::{self, SerialPort};
use anyhow::Result;

pub fn connect() -> Result<Box<dyn SerialPort + 'static>> {
    let port_name: String = std::env::var("SERIAL_PORT_NAME").expect("SERIAL_PORT environment variable not set");
    let baud_rate: u32 = std::env::var("SERIAL_BAUD_RATE")
        .unwrap_or_else(|_| "9600".to_string())
        .parse()
        .expect("Invalid baud rate");

    let port: Box<dyn SerialPort + 'static> = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()?;

    Ok(port)
}
