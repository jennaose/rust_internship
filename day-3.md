# Data Types in Rust

Data types define the kind of values a variable can hold in programming. They specify what kind of operations can be performed on the data and how the data is stored in memory. Different programming languages have various data types, but most share common categories.
Rust programming recognises a numbber of data types for different values
The major data types includes 
- Scalar data type
- Aggregate data type

## Scalar Data type
This data type defines single data values. Rust has four primary scalar types: integers(i), floating-point numbers(f), Booleans(bool), and characters(char). 

-**Integer:** Integer is a single whole number value number. it is a numbber without a fractional part. Declaring this data type is done by attaching i with the size of the integer value i.e. `i32` or `i64` which are 32 bits and 64 bits in size, indicating that the number is a signed integer rather than an unsigned integer which is indicated using a u instead of i.

| Length    | Signed  | Unsigned |
|-----------|---------|----------|
| 8-bit     | i8      | u8       |
| 16-bit    | i16     | u16      |
| 32-bit    | i32     | u32      |
| 64-bit    | i64     | u64      |
| 128-bit   | i128    | u128     |

-**Floating-point numbers:** Floating-point numbers represent real numbers that can have fractional parts (i.e., numbers with decimals). They are used when the precision of whole numbers (integers) is not sufficient, allowing you to represent values between integers as well as very large or very small numbers. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size.

            let y: f32 = 3.0; // f32

this line of code assigns `3.0` fractional number to `y` variable with data type `float` and size 32 bits 

- **Boolean:** this is a data type used when a true or a false value is needed. it is defined using the `bool` keyword

            let heisthere : bool= true;

- **Character:** This is used when declaring alphabetic values. It can also be in declared as symbols.
  
            let c = 'z';
            let z: char = 'ℤ';

## Aggregate Data Type
Aggregate data types are data types that group together multiple individual elements into a single entity. Aggregate types are used when you want to store and manipulate a collection of related data as a single unit.

Here are the common types of aggregate data types:

- **Array:** An array is a collection of elements, all of which are of the same type, stored in contiguous memory locations. Each element in the array can be accessed by its index.

               let numbers: [i32; 5] = [1, 2, 3, 4, 5];  // An array of 5 integers
               println!("{}", numbers[0]);  // Access the first element (1)
  
- **Tuple:** A tuple is a collection of values that can be of different types. Tuples are ordered and can be accessed by position (index). Tuples are commonly used when you want to group together different types of data.

               let person: (i32, f64, &str) = (30, 72.5, "Alice");
               println!("Name: {}, Age: {}, Weight: {}", person.2, person.0, person.1);
  
- **Struct:** A struct (short for "structure") is a custom data type that allows you to group related data using named fields.
  
                        struct Person {
                            name: String,
                            age: u32,
                            height: f64,
                        }
                        
                        let person = Person {
                            name: String::from("Alice"),
                            age: 30,
                            height: 5.5,
                        };
                        println!("Name: {}, Age: {}, Height: {}", person.name, person.age, person.height);
  
- **String:** In some languages (like Rust or C++), a String is considered an aggregate data type because it is essentially a collection of characters (an array of char)

- **Collections:** (**Vectors**, Lists, Hash Maps, etc.) Some aggregate data types are part of the standard library in modern programming languages.
  
                        let mut numbers = vec![1, 2, 3];  // A vector in Rust
                        numbers.push(4);  // Add an element

## Accessing Array Elements
An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. You can access elements of an array using indexing, like this:

                        fn main() {
                            let a = [1, 2, 3, 4, 5];
                        
                            let first = a[0];
                            let second = a[1];
                        }
                          
