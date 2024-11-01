# Rust Error Handling with Result and Option
In Rust, error handling is an essential part of building reliable software. Instead of using exceptions, Rust leverages the Result and Option types to handle scenarios where something might go wrong or where a value might be missing. This approach encourages safer, more predictable code by handling errors as part of the type system itself.

## Result: Handling Recoverable Errors
The Result type is an enum used to represent operations that may either succeed or fail. It’s defined as:

          enum Result<T, E> {
              Ok(T),
              Err(E),
          }
1. Ok(T) holds the value if the operation succeeds.
2. Err(E) holds an error if the operation fails.
   
#### Example

Suppose we want to open a file. This action could fail if the file doesn't exist, so we use Result to handle the potential error:

          use std::fs::File;
          
          fn open_file() -> Result<File, std::io::Error> {
              File::open("file.txt")
          }
          
Here, open_file returns a Result<File, std::io::Error>. We can then use match to handle both cases:

          match open_file() {
              Ok(file) => println!("File opened successfully!"),
              Err(e) => eprintln!("Failed to open file: {}", e),
          }
          
Or, with Rust’s `?` operator, which simplifies error handling by returning early if an error occurs:

          fn read_file() -> Result<String, std::io::Error> {
              let mut file = File::open("file.txt")?;
              let mut contents = String::new();
              file.read_to_string(&mut contents)?;
              Ok(contents)
          }
          
## Option: Handling Absence of a Value

The Option type is for scenarios where a value might or might not be present, without it necessarily being an error. It's defined as:

          enum Option<T> {
              Some(T),
              None,
          }
          
1. Some(T) holds a value if it exists.
2. None represents the absence of a value.

#### Example
Let’s say we want to retrieve a value from a HashMap. The value may or may not exist, so we use Option:
          
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
          
Or, use unwrap_or for a default value if None is encountered:

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
