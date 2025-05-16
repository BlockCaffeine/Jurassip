use serialport::SerialPort;
use anyhow::Result;

use crate::serial::connection::connect;
use crate::serial::send_command::send_command;

pub fn machine_on() -> Result<()> {
    let mut port: Box<dyn SerialPort + 'static> = connect()?;

    // Send the command to turn on the machine
    send_command(&mut port, "AN:01")?;

    Ok(())
}

pub fn machine_off() -> Result<()> {
    let mut port: Box<dyn SerialPort + 'static> = connect()?;

    // Send the command to turn off the machine
    send_command(&mut port, "AN:02")?;

    Ok(())
}
