use crate::serial::send_command::send_command;
use serialport::SerialPort;

// Define an enum for coffee type
pub enum CoffeeType {
    Single,
    Double,
}

// Define an enum for coffee strength
pub enum CoffeeStrength {
    Mild,
    Normal,
    Strong,
    Extra,
}

// Define a struct to hold the parameters for making coffee
pub struct CoffeeParameters {
    pub coffee_type: CoffeeType,
    pub strength: CoffeeStrength,
}

// Function to make coffee
pub fn make_coffee(port: &mut Box<dyn SerialPort>, params: CoffeeParameters) {
    let command = match params.coffee_type {
        CoffeeType::Single => "FA:09",
        CoffeeType::Double => "FA:0A",
    };

    let strength = match params.strength {
        CoffeeStrength::Mild => 1,
        CoffeeStrength::Normal => 2,
        CoffeeStrength::Strong => 3,
        CoffeeStrength::Extra => 4,
    };

    println!("Command: {}, Strength: {}", command, strength);

    for _ in 0..strength {
        send_command(port, "FA:05");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    send_command(port, command);

    println!("Your coffee is ready!");
}
