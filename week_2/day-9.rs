#[derive (Debug)]
enum Category{
    Electronics,
    Clothing,
    Food,
}

#[derive (Debug)]
struct Item{
    name: String,
    category: Category,
    quantity:u32,
}

impl Item{
    fn new (name:&str, category: Category, quantity:u32)-> Item{
        Item{name:name.to_string(),
        category,
        quantity
    }
    }

    fn update_quantity(&mut self, amount:u32) {
        self.quantity += amount;
    }
}

fn main (){
    let mut item1= Item::new("Phone", Category:: Electronics, 15);
    let mut item2= Item::new("Televison", Category:: Electronics, 20);
    let mut item3= Item::new ("Skirt", Category:: Clothing, 7);
    let mut item4= Item::new ("Bread", Category:: Food, 10);

    // print the initial quantity of each item
    println!("Before update: \n{:?}, \n{:?}, \n{:?}, \n{:?}", item1, item2, item3, item4);

    // update quantity of each item
    item1.update_quantity(5);
    item2.update_quantity(10);
    item3.update_quantity(3);
    item4.update_quantity(2);

    // print the updated quantity of each item
    println!("\nAfter update: \n{:?}, \n{:?}, \n{:?}, \n{:?}", item1, item2, item3, item4);
}
/* the result will be:
Before update: 
Item { name: "Phone", category: Electronics, quantity: 15 },
Item { name: "Televison", category: Electronics, quantity: 20 },
Item { name: "Skirt", category: Clothing, quantity: 7 },
Item { name: "Bread", category: Food, quantity: 10 }
After update:
Item { name: "Phone", category: Electronics, quantity: 20 },
Item { name: "Televison", category: Electronics, quantity: 30 },
Item { name: "Skirt", category: Clothing, quantity: 10 },
Item { name: "Bread", category: Food, quantity: 12 }*/
