# Variables in Rust

A variable is a place holder for a value.

In Rust, variables are defined using the `let` syntax and variables are immuatble by default that is they cannot be changed once declared.

Hence if you want to be able to change the contents of a variable, the `mut` keyword is used, preffixing the `let` keyword and so an mutable variable can be defined.

Assume we have a line of code

                let age= 10;

Here `age` cannot be changed, as rust automatically sets variales to immutable 
                     
`let` is used to declare a variable in Rust.

`age` is the name of the variable.

`10` is the value we stored in the variable age.

Assume we have another line of code 

               let mut age= 10
                    
`mut` makes the variable `age` changeable such that we can define `age` again and change the value 

Hence we can have 

                let mut age= 10
                //now to change the value of age 
                let age = 11
                // the age changes from 10 to 11
