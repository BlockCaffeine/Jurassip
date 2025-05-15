// Define an enum for coffee type
#[allow(dead_code)]
pub enum ProductType {
    CoffeeSingle,
    CoffeeDouble,
    EspressoSingle,
    EspressoDouble,
}

// Define an enum for coffee strength
#[allow(dead_code)]
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
