mod api;
mod serial;
mod protocol;

use dotenv::dotenv;
use crate::api::make_product;

fn main() {
    dotenv().ok(); // Load environment variables from .env file

    if let Err(e) = make_product::make_coffee(
        api::product_definitions::CoffeeParameters {
            coffee_type: api::product_definitions::ProductType::CoffeeSingle,
            strength: api::product_definitions::ProductStrength::Normal,
        },
    ) {
        eprintln!("Error making product: {}", e);
    }
}
