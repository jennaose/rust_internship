# STRUCT
A struct is a way of grouping multiple related values into a single entity. Structs allow you to organize your data by grouping fields (values of different types) into a meaningful structure.

## Types of Struct

There are three main types of structs in Rust:

1. Named field struct (classic struct)

2. Tuple struct

3. Unit-like struct

### Named field struct (classic struct)
  
This is the most basic type of struct and it involves defining the struct and then creating instances of the struct. It has named fields, and each field can have a different type. 

**Defining a Tuple Struct:**

                struct Car{
                    model: String,
                    make: String,
                    year: u32,
                    mileage:f64,
                }

#### Here, we defined a Car struct with four fields:

- model (a String)
- make (a String)
- year (a u32, which is an unsigned 32-bit integer)         
- mileage (a f64, which is an float 64-bit integer)

                fn main (){
                    let mut car1 = Car{ //You can also modify fields if the struct is mutable:
                        model: "BMW".to_string(),
                        make: String::from("Camry"),
                        year: 2021,
                        mileage: 1000.444
                    };
                
                    car.year= 2022; //The car1 is mutable so every attribute of the car is mutable

**You can access individual fields of a struct using dot notation:**

                    println!("{} {} {} {}", car.model, car.make, car.year, car.mileage)
                }

### Tuple struct

A tuple in Rust is a collection of values grouped together into a single compound type. A tuple struct in Rust is like a hybrid between a regular struct and a tuple. It allows you to group multiple pieces of data together, but unlike regular structs, the fields in a tuple struct are not named. Instead, the fields are accessed by their position, similar to how you access elements in a tuple.

**Defining a Tuple Struct:**

                struct Colour(i32, i32, i32);

**Creating an Instance:**

                let black = Colour(0, 0, 0);

**You access the fields by index, similar to how you access tuple elements:**

                println!("Black colour values: {} {} {}", black.0, black.1, black.2);

### Unit-like Struct
A unit-like struct is a struct without any fields. You can think of it as a marker or a placeholder type.  It's used when you need to define a type with custom behavior without associating any data with it.

A unit-like struct looks like this:

				struct MyStruct;

Notice that there are no fields or curly braces. It's an empty struct, but it still has a distinct type. This type can be used for various purposes, like implementing methods or traits, or simply as a marker.

**Example**

          struct Empty;
          fn main() {
              let e = Empty;
              println!("Unit-like struct created!");
          }

In this case, the Empty struct does not hold any data, but you can still instantiate it and use it as a type.

