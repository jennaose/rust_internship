# Rust Error Handling with Result and Option
In Rust, error handling is an essential part of building reliable software. Instead of using exceptions, Rust leverages the Result and Option types to handle scenarios where something might go wrong or where a value might be missing. This approach encourages safer, more predictable code by handling errors as part of the type system itself.

## Result: Handling Recoverable Errors
In Rust, the Result type is used to handle operations that can either succeed or fail. It’s an enum that has two variants: Ok and Err. This approach enforces error handling, making code safer by ensuring that errors are addressed instead of ignored.

### Structure of Result
The Result enum is defined as follows:

          enum Result<T, E> {
              Ok(T),
              Err(E),
          }
          
1. Ok(T) holds the value if the operation succeeds.
2. Err(E) holds an error if the operation fails.

#### Common Use Cases
The Result type is commonly used for:

1. File I/O operations (reading/writing files)
2. Network requests
3. Data parsing (converting strings to numbers, for example)

#### Example: Using Result
Let’s look at an example where we attempt to parse a string into an integer.

                              fn main() {
                                  let input = "42";
                                  match input.parse::<i32>() {
                                      Ok(num) => println!("Parsed number: {}", num),
                                      Err(e) => println!("Failed to parse input: {}", e),
                                  }
                              }
- input.parse::<i32>() returns a Result<i32, ParseIntError>.
- If parsing succeeds, it will return Ok with the parsed integer.
- If it fails, it will return Err with an error (ParseIntError).
  
The match expression handles both cases:
- If Ok(num), it prints the parsed number.
- If Err(e), it prints the error message.

### Working with Result Methods
Rust provides several methods for working with Result values without using match explicitly:

1. unwrap(): Returns the value in Ok or panics if it's an Err.

                              let num = "42".parse::<i32>().unwrap();  // Will panic if parsing fails
   
2. expect(): Like unwrap(), but allows a custom error message on panic.

                              let num = "42".parse::<i32>().expect("Failed to parse number");

3. unwrap_or(): Returns the Ok value, or a default if it’s Err.

                              let num = "not a number".parse::<i32>().unwrap_or(0);  // Defaults to 0 if parsing fails

4. is_ok() and is_err(): Checks if the Result is Ok or Err.

                              if result.is_ok() {
                                  println!("Operation succeeded");
                              }

Example: File Reading with Result
rust
Copy code
use std::fs::File;
use std::io::Error;

fn read_file() -> Result<String, Error> {
    let file = File::open("data.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file() {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}
In this example, File::open("data.txt") returns a Result<File, Error>.
If the file is opened successfully, it returns Ok(File).
If it fails, it returns Err(e), where e is the error.
          
## Option: Handling Absence of a Value

The Option type is for scenarios where a value might or might not be present, without it necessarily being an error. It's defined as:

          enum Option<T> {
              Some(T),
              None,
          }
          
1. Some(T) holds a value if it exists.
2. None represents the absence of a value.

#### Example
Let’s say we want to retrieve a value from a `HashMap`. The value may or may not exist, so we use Option:
          
          use std::collections::HashMap;
          
          fn find_in_map(map: &HashMap<String, String>, key: &str) -> Option<&String> {
              map.get(key)
          }
   
We can handle the Option result with match:

          let mut my_map = HashMap::new();
          my_map.insert("key1".to_string(), "value1".to_string());
          
          match find_in_map(&my_map, "key1") {
              Some(value) => println!("Found: {}", value),
              None => println!("Key not found"),
          }
          
Or, use `unwrap_or` for a default value if None is encountered:

          let result = find_in_map(&my_map, "key2").unwrap_or(&"default".to_string());
          println!("Result: {}", result);
          
## Combining Result and Option
Result and Option can be used together to cover both missing values and potential errors. For instance, we might have a function that can fail to retrieve a value due to a missing key or a data processing error.

Here’s an example that tries to fetch and parse an integer from a HashMap:

          fn get_number_from_map(map: &HashMap<String, String>, key: &str) -> Result<Option<i32>, std::num::ParseIntError> {
              match map.get(key) {
                  Some(value) => match value.parse::<i32>() {
                      Ok(num) => Ok(Some(num)),
                      Err(e) => Err(e),
                  },
                  None => Ok(None),
              }
          }
