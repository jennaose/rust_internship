# Creating A New Project Using Cargo
**Cargo** is the official package manager and build system

- Open the terminal on the computer
- Navigate to the directory where you want to save the project
  
          example C:\Users\osegh>cargo new project
  
- Use the following command
   
          cargo new project

  to create a new cargo project to create a new project called project that will have the following structure

          project/         
          src/
          Cargo.toml
          main.rs
          
**Cargo.toml:** A  configuration file used by Cargo, the package manager for Rust, to define project metadata, dependencies, and build settings

**src/main.rs:**  this is the main Rust source file for where the code will be written

## Navigate to the project folder

Move into the newly created project directory using the following command

          cd project	
          
  this opens the new project in visual studio code
        
Cargo already generates the default “Hello, World” program from the previous day and the default content will look like this

           fn main() {
          println!("Hello World!");
          }
          
To compile and run the project, use the following commands

          rustc main.rs
          
Press enter then type 

          ./main
          
And press enter

The following output will be displayed on the terminal

          Compiling maggot v0.1.0 (C:\Users\osegh\project)
              Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s
               Running `C:\Users\osegh\maggot\target\debug\project.exe`
          Hello, World!


