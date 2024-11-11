# TRAITS 
Traits are used to define a set of method signatures that a type must implement. It is a way to define shared behaviour across different types.
Think of them as a contract or a set of rules that a type can promise to follow.
It is different from enum and structs such that it is like an interface that defines functionality a type must implement if it claims to implement the trait.
## Defining a Trait
You define a trait with the trait keyword, followed by one or more method signatures. Here’s a simple example:
```rust
trait Describable {
    fn describe(&self) -> String;
}
```
In this example, a method names describe returns a String and it operates on a borrowed reference of an instance.

## Implementing a Trait
To use a trait on a type, you must implement it for that type. Here’s how:
```rust
struct Person {
    name: String,
    age: u8,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old.", self.name, self.age)
    }
}
```
Now, the Person struct implements the Describable trait, meaning it has a describe method.

## Using Traits with Generic Types
Generic data type is a way of writing code such that it can handle different data types and traits. Traits make Rust’s generic types more powerful. By adding trait bounds, you can specify that a generic type parameter must implement certain traits.

For example, here’s a function that only works on types that implement **Describable**:
```rust
fn print_description<T: Describable>(item: &T) {
    println!("{}", item.describe());
}
```
Now you can call `print_description` on any type that implements **Describable**:
```rust
let person = Person { name: "Alice".to_string(), age: 30 };
print_description(&person);  // Output: Alice is 30 years old.
```
## Default Implementations in Traits
You can provide default implementations for trait methods, which types can use without needing to implement them from scratch.
```rust
trait Describable {
    fn describe(&self) -> String {
        String::from("This is something describable.")
    }
}
```
Any type that implements **Describable** will now inherit this default describe method unless it overrides it with its own implementation.

## Commonly Used Traits
Rust includes many built-in traits, like Clone, Copy, Debug, PartialEq, and Display. Implementing these traits gives your types additional functionality:

- **Clone**: Enables deep copying of values.
- **Debug**: Allows printing the type in a debug format with {:?}.
- **Display**: Allows printing the type in a user-friendly way with {}.

#### Example: Debug Trait
Implementing the Debug trait lets you easily print a struct using {:?} for debugging:
```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 20 };
    println!("{:?}", point);  // Output: Point { x: 10, y: 20 }
}
```
### Summary
- Traits define shared behavior across types.
- Implementing a trait on a type allows that type to use the trait's methods.
- You can add trait bounds to generic types to enforce certain behaviors.
- Rust provides default implementations for many useful built-in traits.
