use std::io;
use std::io::Write;

#[derive (Debug)]
struct Expense {
    description: String,
    quantity: i64,
    price: f64,
    date: String,
}
impl Expense {
    fn new (description: String, quantity: i64, price: i64, date: String)-> Self{
        Self { description, quantity, price: price as f64, date,
        }
    }

    fn display(&self)-> String {
        format!("{}:{}, Price: ${:.2}, Date:{}", self.description, self.quantity, self.price as f64, self.date)
    }

    fn total(&self) -> f64 {
        self.quantity as f64 * self.price
    }
}

fn main(){
    let mut expenses : Vec<Expense>= Vec::new();

    loop {
        println!("\n Simple Expense Tracker");
        println!("1. Add an expense");
        println!("2. Show all expenses");
        println!("3. Calculate total expenses");
        println!("4. Exit");

        print!("Enter what you wish to do (1-4):");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line( &mut choice).expect ("Failed to read line");
        let choice=choice.trim();

        match choice {
            "1" => add_expense(&mut expenses),
            "2" => show_expenses(&expenses),
            "3" => calculate_total(&expenses),
            "4" => {
                println!("Goodbye");
                break;
            }
        _=> println!("Invalid choice. Please try again"),
        }
    }
}
 fn add_expense(expenses: &mut Vec<Expense>){
    let description = get_input (" Enter Expense Description");
    let quantity : i64 = get_input ("Enter Expense quantity")
        .trim().parse().expect("Invalid quantity");
    let price : i64 = get_input ("Enter Expense Price. Enter a whole number")
        .trim().parse().expect("Invalid price");
    let date = get_input ("Enter Expense Date")
        .trim().parse().expect("Invalid date");

    let expense = Expense::new(description, quantity, price, date);
    expenses.push(expense);
    println!("Expense added successfully");
 }
 fn show_expenses(expenses:&[Expense]){
    if expenses.is_empty(){
        println!("No expenses recorded");
    } else {
        println!("your expenses");
        for (i,expense) in expenses.iter().enumerate(){
        println!("{}.{}", i + 1, expense.display()) 
        }
    }
 }

 fn calculate_total(expenses: &[Expense]) {
    let mut grand_total = 0.0;

    if expenses.is_empty() {
        println!("No expenses recorded.");
    } else {
        println!("Expense details and individual totals:");
        for (i, expense) in expenses.iter().enumerate() {
            let item_total = expense.total();
            println!("{}. {} - Total: ${:.2}", i + 1, expense.description, item_total);
            grand_total += item_total;
        }
        println!("\nGrand Total of all expenses: ${:.2}", grand_total);
    }
}

fn get_input(prompt: &str)-> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
