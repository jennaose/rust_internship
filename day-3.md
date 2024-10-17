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
