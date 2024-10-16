# Variables in Rust

A variable is a place holder for a value.

In Rust, variables are defined using the `let` syntax and variables are immuatble by default that is they cannot be changed once declared.

Hence if you want to be able to change the contents of a variable, the `mut` keyword is used, preffixing the `let` keyword and so an mutable variable can be defined.

## Create a new cargo project
          fn main() {
           //immutable variables
           let age= 10;
           //this is automatically an immutable  variable 
           
          println!("age {} ", age);
          // mutable variables
           let mut age2= 16;
           println!("before= {} ", age2);
           age2= 20;
           println!("after= {} ", age2)
          }

In the first instance, **age** is an immutable variable so it cannot be changed once assigned.

But in the second instance, **age2** is a muttable variable, hence it could be changed affter initial assignment.
