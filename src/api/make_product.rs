use crate::serial::send_command::send_command;
use serialport::SerialPort;

// Define an enum for coffee type
pub enum ProductType {
    CoffeeSingle,
    CoffeeDouble,
    EspressoSingle,
    EspressoDouble,
}

// Define an enum for coffee strength
pub enum ProductStrength {
    Mild,
    Normal,
    Strong,
    Extra,
}

// Define a struct to hold the parameters for making coffee
pub struct CoffeeParameters {
    pub coffee_type: ProductType,
    pub strength: ProductStrength,
}

// Function to make coffee
pub fn make_coffee(port: &mut Box<dyn SerialPort>, params: CoffeeParameters) {
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

    for _ in 0..strength {
        send_command(port, "FA:05");
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    send_command(port, command);

    println!("Your product is ready!");
}
