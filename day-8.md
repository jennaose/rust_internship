# ENUMS
**Enums**, short for enumerations, are a type in Rust that allow you to define a value that can be one of several distinct variants. Enums are used to represent data that can have multiple possible states, where each state

### Using Enums
You can create a variable of an enum type by assigning it to one of the enum’s variants. Then, you can use a match expression or an if let statement to handle each variant. may have its own unique structure or no additional data at all.

**Basic Syntax**

Here’s how you define an enum in Rust:

          enum Direction {
              North,
              South,
              East,
              West,
          }

### Enums with Data
Enums in Rust can also store additional data associated with each variant, allowing more complex structures.

Example:

          enum Message {
              Quit,
              Move { x: i32, y: i32 },
              Write(String),
              ChangeColor(i32, i32, i32),
          }

Here, Message has four variants:

1. **Quit**: A simple variant with no associated data.
2. **Move { x: i32, y: i32 }**: A variant with two named fields (x and y), representing coordinates.
3. **Write(String)**: A variant with a single associated String value, representing a text message.
4. **ChangeColor(i32, i32, i32)**: A variant with three i32 values, which can be used to represent RGB color values.

### Example with Pattern Matching

You can use match to access the data inside enum variants:

          fn main() {
              let msg = Message::Move { x: 10, y: 20 };
          
              match msg {
                  Message::Quit => println!("Quit message"),
                  Message::Move { x, y } => println!("Move to coordinates ({}, {})", x, y),
                  Message::Write(text) => println!("Text message: {}", text),
                  Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
              }
          }

We use match msg to check which variant of Message is currently stored in msg. The match expression then runs different code based on the variant.

Each arm of the match expression handles one of the four Message variants:

1. **Message::Quit**: Prints "Quit message".
2. **Message::Move { x, y }**: Destructures the Move variant to access its x and y values and prints them.
3. **Message::Write(text)**: Extracts the text value from Write and prints it.
4. **Message::ChangeColor(r, g, b)**: Destructures ChangeColor to access the r, g, and b values and prints them in an RGB format.

### What the Code Does
Since msg is set to `Message::Move { x: 10, y: 20 }`, Rust goes through the match arms and finds the `Message::Move { x, y }` pattern. It then:

Matches this pattern and extracts the values of x and y.

Prints Move to coordinates (10, 20).
