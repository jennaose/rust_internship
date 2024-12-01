use std::collections::HashMap;
use std::io; // Import for input/output functionality

#[derive(Debug)]
// Define the Contact struct
struct Contact {
    name: String,
    phone: String,
    email: String,
}

// Implement methods for Contact struct
impl Contact {
    fn new(name: &str, phone: &str, email: &str) -> Contact {
        Contact {
            name: name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
        }
    }
}

#[derive(Debug)]
// Define the ContactBook struct
struct ContactBook {
    contacts: Vec<Contact>,
    index: HashMap<String, usize>, // Maps names to positions in the `contacts` vector
}

// Implement methods for ContactBook struct
impl ContactBook {
    fn new() -> ContactBook {
        ContactBook {
            contacts: Vec::new(),
            index: HashMap::new(),
        }
    }

    // Add a new contact
    fn add_contact(&mut self, contact: Contact) {
        self.index.insert(contact.name.clone(), self.contacts.len());
        self.contacts.push(contact);
    }

    // Remove the first contact (front of the list)
    fn remove_contact_by_name(&mut self, name: &str) -> Option<Contact> {
        if let Some(pos) = self.index.get(name).copied() { // Use `copied()` to get the value

        let removed_contact=self.contacts.remove(pos);// remove the contact at the specified position
        self.index.remove (name);//remove the name from index
        self.update_index();//update the index after removing a contact
        Some(removed_contact)
        } else{
            None
        }
    }

    // Update index after removing a contact to keep it in sync
   fn update_index(&mut self) {
    self.index.clear();
    for (i, contact) in self.contacts.iter().enumerate(){
        self.index.insert(contact.name.clone(), i);
    }
   }

    // Find a contact by name
    fn find_contact(&self, name: &str) -> Option<&Contact> {
        self.index.get(name).map(|&pos| &self.contacts[pos])
    }
}

fn main() {
    let mut contact_book = ContactBook::new();

    // Creating and adding contacts
    let contact1 = Contact::new("Jennifer", "09037942926", "oseghalejennifer13@gmail.com");
    let contact2 = Contact::new("Leonard", "08108017629", "oseghaleleonard13@gmail.com");
    let contact3 = Contact::new("Gloria", "08033333333", "oseghalegloria@gmail.com");
    let contact4 = Contact::new("David", "08034444333", "oseghaledavid@gmail.com");

    contact_book.add_contact(contact1);
    contact_book.add_contact(contact2);
    contact_book.add_contact(contact3);
    contact_book.add_contact(contact4);

    // Print all contacts in the contact book
    println!("These are the contacts in the contact book: \n");
    for contact in &contact_book.contacts {
        println!("{:?}", contact);
    }

    // Get user input for the name to search
    println!("\n Enter the name of the contact you want to find:");
    let mut search_name = String::new();
    io::stdin().read_line(&mut search_name).expect("Failed to read line");

    // Remove any extra whitespace (like newline character)
    let search_name = search_name.trim();

    // Find the contact dynamically based on user input
    match contact_book.find_contact(search_name) {
        Some(contact) => println!("\n Found contact: {:?}", contact),
        None => println!("\n Contact not found"),
    }

    println!("Enter the name of the contact you want to remove:");
    let mut name_to_remove = String::new();
    io::stdin().read_line(&mut name_to_remove).expect("Failed to read line");
    let name_to_remove = name_to_remove.trim();
    
    match contact_book.remove_contact_by_name(name_to_remove) {
        Some(contact) => println!("Successfully removed contact:\nName: {}\nPhone: {}\nEmail: {}", 
                                   contact.name, contact.phone, contact.email),
        None => println!("Contact '{}' not found. Please check the name and try again.", name_to_remove),
    }

    // Update the index after removal
    contact_book.update_index();

    // Print remaining contacts
    println!("\n After removing the contact:");
    for contact in &contact_book.contacts {
        println!(" {:?}", contact);
    }
}

