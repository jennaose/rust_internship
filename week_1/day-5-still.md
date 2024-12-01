# Simple Command Line Interface Calculator in Rust
This is a command-line calculator built with Rust that performs basic arithmetic operations: addition, subtraction, multiplication, and division. The program takes two numbers as input and allows the user to specify which operation to perform.

## Features
- Addition (+)
- Subtraction (-)
- Multiplication (*)
- Division (/): Division by zero is handled with an error message.

## Usage
- Clone the repository:

(as it is in my computer)

            git clone https://github.com/jennaose/rust_internship.git
            rustc day-5-calculator.rs

- Run the program:

            day-5-calculator.exe
      
- Follow the on-screen prompts to:

            Enter the first number
            Enter the second number
            Specify the operation you want to perform (+, -, *, or /)
Example

            Enter the first number:
            5
            Enter the second number:
            3
            Enter the operation (+, -, *, /):
            +
            The sum is: 8
            
For division by zero:

            Enter the first number:
            5
            Enter the second number:
            0
            Enter the operation (+, -, *, /):
            /
            Cannot divide by zero.

## How It Works
- The program takes two numbers and an operation from the user.
- Based on the operation selected, it performs the respective arithmetic operation.
- If division is selected and the second number is zero, the program will notify the user that division by zero is not allowed.
