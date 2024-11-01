// Adding Add error handling ( result and option) to the inventory system, allowing for item categorization and error handling.
#[derive(Debug)]
enum Category {
    Electronics,
    Clothing,
    Food,
}

#[derive(Debug)]
struct Item {
    name: String,
    category: Category,
    quantity: u32,
}

impl Item {
    // Constructor for creating a new item.
    fn new(name: &str, category: Category, quantity: u32) -> Item {
        Item {
            name: name.to_string(),
            category,
            quantity,
        }
    }

    // Updates the quantity with error handling.
    fn update_quantity(&mut self, amount: i32) -> Result<(), String> {
        // Allow positive amounts to increase quantity and negative amounts to decrease it.
        if amount < 0 && self.quantity < amount.unsigned_abs() {
            Err(format!("Cannot decrease {} by {}. Not enough in stock!", self.name, amount.abs()))
        } else {
            // Safely add or subtract quantity.
            self.quantity = (self.quantity as i32 + amount) as u32;
            Ok(())
        }
    }

    // Categorizes the item by its type.
    fn categorize(&self) {
        match self.category {
            Category::Electronics => println!("{:?} is an electronic item.", self),
            Category::Clothing => println!("{:?} is a clothing item.", self),
            Category::Food => println!("{:?} is a food item.", self),
        }
    }
}

// The Inventory struct holds items and provides methods for interacting with them.
struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    // Creates a new inventory.
    fn new() -> Inventory {
        Inventory { items: Vec::new() }
    }

    // Adds an item to the inventory.
    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    // Finds an item by name, returning an Option.
    fn find_item(&mut self, name: &str) -> Option<&mut Item> {
        self.items.iter_mut().find(|item| item.name == name)
    }

    // Attempts to update an item's quantity with error handling.
    fn update_item_quantity(&mut self, name: &str, amount: i32) -> Result<(), String> {
        match self.find_item(name) {
            Some(item) => item.update_quantity(amount),
            None => Err(format!("Item '{}' not found in inventory.", name)),
        }
    }
}

fn main() {
    // Initialize inventory and add items.
    let mut inventory = Inventory::new();
    inventory.add_item(Item::new("Phone", Category::Electronics, 15));
    inventory.add_item(Item::new("Television", Category::Electronics, 20));
    inventory.add_item(Item::new("Skirt", Category::Clothing, 7));
    inventory.add_item(Item::new("Bread", Category::Food, 10));

    // Print initial inventory.
    println!("Before update:");
    for item in &inventory.items {
        println!("{:?}", item);
    }

    // Attempt to update quantities with error handling.
    println!("\nUpdating quantities:");
    match inventory.update_item_quantity("Phone", 5) {
        Ok(_) => println!("Successfully updated Phone quantity."),
        Err(e) => println!("Error: {}", e),
    }
    match inventory.update_item_quantity("Television", -5) {
        Ok(_) => println!("Successfully updated Television quantity."),
        Err(e) => println!("Error: {}", e),
    }
    match inventory.update_item_quantity("Skirt", 3) {
        Ok(_) => println!("Successfully updated Skirt quantity."),
        Err(e) => println!("Error: {}", e),
    }
    match inventory.update_item_quantity("Bread", -12) {
        Ok(_) => println!("Successfully updated Bread quantity."),
        Err(e) => println!("Error: {}", e),
    }
    match inventory.update_item_quantity("Nonexistent", 10) {
        Ok(_) => println!("Successfully updated Nonexistent item quantity."),
        Err(e) => println!("Error: {}", e),
    }

    // Print updated inventory.
    println!("\nAfter update:");
    for item in &inventory.items {
        println!("{:?}", item);
    }

    // Categorize each item.
    println!("\nCategorizing items:");
    for item in &inventory.items {
        item.categorize();
    }
}
