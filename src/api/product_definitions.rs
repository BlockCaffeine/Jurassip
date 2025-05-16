// Define an enum for coffee type
#[allow(dead_code)]
pub enum ProductType {
    SingleCoffee,
    DoubleCoffee,
    SingleEspresso,
    DoubleEspresso,
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
pub struct ProductParameters {
    pub product_type: ProductType,
    pub product_strength: ProductStrength,
}
