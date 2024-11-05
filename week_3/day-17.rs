// this is an updated contactbook with vector method included 

use std::collections::HashMap;

#[derive(Debug)]
struct Contact {
    name: String,
    phone: String,
    email: String,
}

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
struct ContactBook {
    contacts: Vec<Contact>,
    index: HashMap<String, usize>,
}

impl ContactBook {
    fn new() -> ContactBook {
        ContactBook {
            contacts: Vec::new(),
            index: HashMap::new(),
        }
    }

    fn add_contact(&mut self, contact: Contact) {
        self.index.insert(contact.name.clone(), self.contacts.len());
        self.contacts.push(contact);
    }

    fn remove_contact(&mut self, name: &str) -> Option<Contact> {
        if let Some(&pos) = self.index.get(name) {
            self.index.remove(name);
            Some(self.contacts.remove(pos))
        } else {
            None
        }
    }

    fn find_contact(&self, name: &str) -> Option<&Contact> {
        self.index.get(name).map(|&pos| &self.contacts[pos])
    }
}

fn main() {
    let mut contact_book = ContactBook::new();

    // Adding contacts
    let contact1 = Contact::new("Alice", "123-456-7890", "alice@example.com");
    let contact2 = Contact::new("Bob", "098-765-4321", "bob@example.com");
    contact_book.add_contact(contact1);
    contact_book.add_contact(contact2);

    // Searching for a contact
    match contact_book.find_contact("Alice") {
        Some(contact) => println!("Found contact: {:?}", contact),
        None => println!("Contact not found"),
    }

    // Removing a contact
    contact_book.remove_contact("Alice");

    // Trying to find the removed contact
    match contact_book.find_contact("Alice") {
        Some(contact) => println!("Found contact: {:?}", contact),
        None => println!("Contact not found"),
    }

    // Printing all contacts
    for contact in &contact_book.contacts {
        println!("{:?}", contact);
    }
}
