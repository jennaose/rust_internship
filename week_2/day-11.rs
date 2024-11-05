// Adding pattern matching to the inventory system, allowing for item categorization.
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
    fn new(name: &str, category: Category, quantity: u32) -> Item {
        Item {
            name: name.to_string(),
            category,
            quantity,
        }
    }

    fn update_quantity(&mut self, amount: u32) {
        self.quantity += amount;
    }

    fn categorize(&self) {
        match self.category {
            Category::Electronics => println!("{:?} is an electronic item.", self),
            Category::Clothing => println!("{:?} is a clothing item.", self),
            Category::Food => println!("{:?} is a food item.", self),
        }
    }
}

fn main() {
    let mut item1 = Item::new("Phone", Category::Electronics, 15);
    let mut item2 = Item::new("Television", Category::Electronics, 20);
    let mut item3 = Item::new("Skirt", Category::Clothing, 7);
    let mut item4 = Item::new("Bread", Category::Food, 10);

    // print the initial quantity of each item
    println!("Before update: \n{:?}, \n{:?}, \n{:?}, \n{:?}\n", item1, item2, item3, item4);

    // update quantity of each item
    item1.update_quantity(5);
    item2.update_quantity(10);
    item3.update_quantity(3);
    item4.update_quantity(2);

    // print the updated quantity of each item
    println!("\nAfter update: \n{:?}, \n{:?}, \n{:?}, \n{:?} \n", item1, item2, item3, item4);

    // categorize each item
    println!("\nCategorizing them" );
    item1.categorize();
    item2.categorize();
    item3.categorize();
    item4.categorize();
}
