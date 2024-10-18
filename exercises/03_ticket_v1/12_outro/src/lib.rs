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
    pub fn new(product_name:String, quantity:u32, unit_price:u32) -> Order{
        // todo!();
        valid_product_name(&product_name);
        valid_quantity(&quantity);
        valid_unit_price(&unit_price);
        // if product_name.is_empty() {
        //     panic!("product_name cannot be empty");
        // }
        // if product_name.len() <= 300 {
        //     panic!("product_name cannot be longer than 50 bytes");
        // }
        // if quantity > 0 {
        //     panic!("quantity cannot be empty");
        // }
        // if unit_price > 0 {
        //     panic!("unit_price cannot be empty");
        // }
        // if description.len() > 500 {
        //     panic!("Description cannot be longer than 500 bytes");
        // }
        // if status != "To-Do" && status != "In Progress" && status != "Done" {
        //     panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        // }
        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn total(&self) -> u32 {
        // let total = &self.unit_price*&self.quantity;
        // &total
        self.unit_price*self.quantity
    }

    pub fn set_product_name(&mut self, new_product_name: String){
        // set new title
        valid_product_name(&new_product_name);
        self.product_name = new_product_name;
    }

    pub fn set_quantity(&mut self, new_quantity: u32){
        valid_quantity(&new_quantity);
        self.quantity = new_quantity;
    }

    pub fn set_unit_price(&mut self, new_unit_price: u32){
        valid_unit_price(&new_unit_price);
        self.unit_price = new_unit_price;
    }

}

fn valid_product_name(product_name: &String){
    if product_name.is_empty() {
        panic!("The product name can't be empty");
    }
    if product_name.len() > 300 {
        panic!("The product name can't be longer than 300 bytes");
    }
    // todo!();
}

fn valid_quantity(quantity: &u32){
    if *quantity == 0 {
        panic!("The quantity must be strictly greater than zero.");
    }
    // todo!();
}

fn valid_unit_price(unit_price: &u32){
    if *unit_price == 0 {
        panic!("The unit price is in cents and must be strictly greater than zero.");
    }
    // todo!();
}
