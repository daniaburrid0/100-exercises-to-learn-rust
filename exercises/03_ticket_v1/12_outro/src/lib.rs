// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    fn validate_product_name(name: &str) {
        if name.len() > 300 {
            panic!("Product name cannot be longer than 300 bytes");
        }
        if name.is_empty() {
            panic!("Product name cannot be empty");
        }
    }

    fn validate_quantity(q: u32) {
        if q == 0 {
            panic!("Quantity must be strictly greater than zero");
        }
    }

    fn validate_unit_price(p: u32) {
        if p == 0 {
            panic!("Unit price must be strictly greater than zero");
        }
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn set_product_name(&mut self, name: String) {
        Self::validate_product_name(&name);
        self.product_name = name;
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        Self::validate_quantity(quantity);
        self.quantity = quantity;
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_unit_price(&mut self, price: u32) {
        Self::validate_unit_price(price);
        self.unit_price = price;
    }

    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        Self::validate_product_name(&product_name);
        Self::validate_quantity(quantity);
        Self::validate_unit_price(unit_price);
        Self {
            product_name,
            quantity,
            unit_price,
        }
    }
}
