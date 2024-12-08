use std::fs::File;
use std::fs::Write;

fn main (){
let mut file= File::create ("note.txt").expect("Failed to create file" );
file.write-all(b"my learning rust journal").expect("Failed to write to file");

println!("Data has been written to file");

let content = fs::read_to_string("note.txt").expect(Failed to read from file");

match content{
Ok(content)=>println!("\nContent:\n{}", content);
Err(e)=>("Error in reading file ");


 
