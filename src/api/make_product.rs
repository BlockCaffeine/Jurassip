use serialport::SerialPort;
use anyhow::Result;

use crate::serial::connection::connect;
use crate::serial::send_command::send_command;

use crate::api::product_definitions::{ProductParameters, ProductType, ProductStrength};

// Function to make coffee
pub fn make_product(params: ProductParameters) -> Result<()> {
    let mut port: Box<dyn SerialPort + 'static> = connect()?;

    let command: &'static str = match params.product_type {
        ProductType::SingleCoffee => "FA:09",
        ProductType::DoubleCoffee => "FA:0A",
        ProductType::SingleEspresso => "FA:07",
        ProductType::DoubleEspresso => "FA:08",
    };

    let strength: i32 = match params.product_strength {
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
