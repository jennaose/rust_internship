use serde_json::Value;
fn main((){
let json_data =r#"{
"name":"Rust",
"type"::"Programming Language"}"#
let parsed : Value = serde_json::from_str(json_data).expect("Failed to parse json");
println!("Name:{}", parsed ["name"]);
println!("Type:{}", parsed ["type"]};
}
/*
The output will be
Name:Rust
Type: Programming Language
*/
