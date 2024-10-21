use std::io;
fn main(){
    //to read the first number
    println!("enter the first number");
    let mut num1=String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read input");//for error handling
    let num1:f64=num1.trim().parse().expect("please enter a valid number");//to convert the number to floating point and check for extra spaces

    //to read the second number
    println!("enter the second number");
    let mut num2= String :: new();
    io:: stdin().read_line(& mut num2).expect ("Failed to read input");//for error handling
    let num2:f64=num2.trim().parse().expect("please enter a valid number");//to convert the number to floating point and check for extra spaces

    //to read the operation the user wants to use 
    println!("enter the operation (+,-,*,/):");
    let mut operation=String:: new ();
    io:: stdin().read_line(& mut operation).expect("Failed to read input");
    let operation=operation.trim();

    //to perform the operation
    match operation{
        "+" => println!("the sum is :{}", num1+num2),
        "-"=> println!("the difference is: {}", num1-num2),
        "*"=> println!("the product is: {}", num1*num2),
        "/"=>{
            if num2 !=0.0{
            println!("the quotient is :{}", num1/num2);
        }
            else {
                println!("cannot be divided by zero.");
            }
        }
        _ =>println!("the operation is invalid. please enter any of the following (+,-,*,/)"),
    }
}
