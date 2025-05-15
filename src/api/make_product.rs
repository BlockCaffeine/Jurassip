use serialport::SerialPort;
use anyhow::Result;

use crate::serial::connection::connect;
use crate::serial::send_command::send_command;

use crate::api::product_definitions::{CoffeeParameters, ProductType, ProductStrength};

// Function to make coffee
pub fn make_coffee(params: CoffeeParameters) -> Result<()> {
    let mut port: Box<dyn SerialPort + 'static> = connect()?;

    let command: &'static str = match params.coffee_type {
        ProductType::CoffeeSingle => "FA:09",
        ProductType::CoffeeDouble => "FA:0A",
        ProductType::EspressoSingle => "FA:07",
        ProductType::EspressoDouble => "FA:08",
    };

    let strength: i32 = match params.strength {
        ProductStrength::Mild => 1,
        ProductStrength::Normal => 2,
        ProductStrength::Strong => 3,
        ProductStrength::Extra => 4,
    };

    println!("Command: {}, Strength: {}", command, strength);

    // Set the strength
    for _ in 0..strength {
        send_command(&mut port, "FA:05")?;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    // Send the command to make coffee or espresso
    send_command(&mut port, command)?;

    Ok(())
}
