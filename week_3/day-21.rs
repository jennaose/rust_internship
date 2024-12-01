use std::collections::HashMap; //  to import the library for using Hashmaps 
use std::io::{self, Write}; // to import the library to accept user input


#[derive(Debug)]// Allows to use the {:?} when printinhg contents
//To create a struct for Contact
struct Contact{
    name: String,
    phone: String,
    email: String,
}
//to implement the Contact struct
impl Contact{
    fn new (name: &str, phone: &str, email: &str)-> Contact {
        Contact{
            name: name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
        }
    }
}

#[derive(Debug)]    // Allows to use the {:?} when printinhg contents
// To create a struct for ContactBook
struct ContactBook{
    contacts : Vec<Contact>,
    index: HashMap<String,usize>,
}
//to implement the struct for ContactBook
impl ContactBook{
    fn new() -> ContactBook{
        ContactBook{
            contacts: Vec::new(),
            index: HashMap::new()
        }
    }

    //To add a new contact
    fn add_new_contact(&mut self, contact:Contact){
        self.index.insert(contact.name.clone(), self.contacts.len());
        self.contacts.push(contact);
    }

    //To remove a contact
    fn remove_contact(&mut self, name:&str)->Option<Contact>{
        if let Some (&pos)=self.index.get(name){
            Some(self.contacts.remove(pos))
        }else{
            None
        }
    }

    //To find a contact
    fn find_contact(&self, name:&str)-> Option<&Contact>{
        self.index.get(name).map(|&pos| &self.contacts[pos])
    }

    //to display contact
    fn display_contacts(&self){
        if self.contacts.is_empty() {
            println!("No contacts in the contact book.");
         } else {
            println!("Contacts in the contact book.");
        for contact in &self.contacts{
                    println!("{:?}", contact)
            }
        }
    }
}

// The main function
fn main(){
    let mut contact_book= ContactBook::new();

    loop {
        println!("\nContact Book Menu:");
        println!("1. Add a contact");
        println!("2. Find a contact");
        println!("3. Remove a contact");
        println!("4. Display all contacts");
        println!("5. Quit");

        println!("\n Please choose an option (1-5):");
        io::stdout().flush().unwrap(); // to male the outputs display immediately

        let mut choice= String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice= choice.trim();

        match choice {
            "1"=> { 
                let mut name = String::new();
                let mut phone = String::new();
                let mut email = String::new();

                println!("Enter the name:");
                io::stdin().read_line(&mut name).expect("Faield to read line");

                println!("Enter the phone number:");
                io::stdin().read_line(&mut phone).expect("Failed to read line");

                println!("Enter the email number:");
                io::stdin().read_line(&mut email).expect("Failed to read line");

                let new_contact= Contact::new(name.trim(), phone.trim(), email.trim());
                contact_book.add_new_contact(new_contact);
                println!("Contact added successfully");
            }

            "2"=> {
                println!("Enter the name of the contact you want to find:");
                let mut name_to_find= String::new();
                io::stdin().read_line(&mut name_to_find).expect("Faield to read line");

                match contact_book.find_contact(name_to_find.trim()){
                    Some(contact)=>println!("Found the contact:{:?}", contact),
                    None=> println!("Contact not Found {:?} ", name_to_find.trim()),
                }
            }

            "3"=> {
                println!("Enter the name of the contact you want to remove:");
                let mut name_to_remove= String::new();
                io::stdin().read_line(&mut name_to_remove).expect("Faield to read line");

                match contact_book.remove_contact(name_to_remove.trim()){
                    Some(contact)=>println!("Found the contact: {:?}", contact),
                    None => println!("Contact not Found {:?} ", name_to_remove.trim()),
                }
            }

            "4"=> {
                contact_book.display_contacts();
            }

            "5"=> {
                print!("Goodbye!");
            break;
        } 
        _ => {
         println!("Invalid option. Please choose valid option (1-5)");
            }
        }
    }
} 
