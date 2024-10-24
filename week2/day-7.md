STRUCT
A struct is a way of grouping multiple related values into a single entity. Structs allow you to organize your data by grouping fields (values of different types) into a meaningful structure.

There are three main types of structs in Rust:

Named field struct (classic struct)
Tuple struct
Unit-like struct

Named field struct (classic struct)
This is the most basic type of struct and it involves defining the struct and then creating instances of the struct. It has named fields, and each field can have a different type. 

Defining a Tuple Struct:

struct Car{
    model: String,
    make: String,
    year: u32,
    mileage:f64,
}

Here, we defined a Car struct with four fields:

model (a String)
make (a String)
year (a u32, which is an unsigned 32-bit integer)
mileage (a f64, which is an float 64-bit integer)

fn main (){
    let mut car1 = Car{ //You can also modify fields if the struct is mutable:
        model: "BMW".to_string(),
        make: String::from("Camry"),
        year: 2021,
        mileage: 1000.444
    };


    car.year= 2022; //The car1 is mutable so every attribute of the car is mutable

You can access individual fields of a struct using dot notation:

    println!("{} {} {} {}", car.model, car.make, car.year, car.mileage)
}

Tuple struct

A tuple in Rust is a collection of values grouped together into a single compound type. A tuple struct in Rust is like a hybrid between a regular struct and a tuple. It allows you to group multiple pieces of data together, but unlike regular structs, the fields in a tuple struct are not named. Instead, the fields are accessed by their position, similar to how you access elements in a tuple.

Defining a Tuple Struct:

struct Colour(i32, i32, i32);

Creating an Instance:

let black = Colour(0, 0, 0);

You access the fields by index, similar to how you access tuple elements:

println!("Black colour values: {} {} {}", black.0, black.1, black.2);
