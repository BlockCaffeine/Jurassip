use std::time::Duration;
use serialport::{self, SerialPort};

pub fn connect(port_name: &str, baud_rate: u32) -> Result<Box<dyn SerialPort + 'static>, Box<dyn std::error::Error>> {
    let port: Box<dyn SerialPort + 'static> = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()?;

    Ok(port)
}
