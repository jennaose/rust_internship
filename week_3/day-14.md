# Vectors in Rust
Vectors in Rust are a resizable array type provided by the standard library. They offer a dynamic array, which means their size can grow or shrink as needed.
#### Creating a vector:

        let mut numbers = vec![1, 2, 3]; // to create a vector field with integers 1,2,3
        
or
        
        let mut vec = Vec::new(); // to create a new mutable empty vector field called vec

#### Adding elements:

        numbers.push(4);
        
This will add 4 to the end of the vector.

or 

        vec.push(1);
        vec.push(2);
        vec.push(3);

This will add integers 1, 2, 3 to the new vector field 

#### Accessing elements:

        let first = numbers[0]; // Accesses the first element

or 

        let first = vec[0];
        
Rust uses zero-based indexing, so numbers[0] gets the first element.

#### Removing Elements:

        vec.pop(); // Removes the last element


#### Looping through vectors:

        for num in &numbers {
            println!("{}", num);
        }

or 

        for i in &vec {
            println!("{}", i);
        }
        
**Why use vectors:** They’re great for when you need to store a list of items where you may want to dynamically adjust the size, like a collection of contacts or scores.


# Hash Maps in Rust
Hash maps are incredibly useful in Rust when you need to store key-value pairs and retrieve values efficiently. You can think of it as a dictionary, where each key is unique, and it’s linked to a specific value.

#### Creating a Hash Map:

        use std::collections::HashMap;
        let mut scores = HashMap::new();

#### Adding Elements:

        scores.insert("Blue", 10);
        scores.insert("Yellow", 50);

This stores "Blue" with a score of 10 and "Yellow" with a score of 50.

#### Accessing Values:
        if let Some(score) = scores.get("Blue") 
        { println!("Blue's score is {}", score);
        }

This approach allows you to directly handle the Some variant of the Option returned by scores.get().
        
#### Updating Values:

        scores.insert("Blue", 25); // Replaces the value for "Blue" from 10 to 25

#### Iterating Over a Hash Map:

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
        
#### Removing Elements:

        scores.remove("Blue");
