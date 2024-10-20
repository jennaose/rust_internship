## Functions in Rust
If you want to perform a task, this task could include steps. Now imagine you have to perform this task over and over again, means you have to repeat these steps included in the task over and over again to achieve a desired result. We can group the steps for this task into one `function` such that if we want to run this task, we could just call on the function and the steps inside the task would be performed.
This is an example of what a function looks like

          fn make_tea() {
              println!("Boil water");
              println!("Add tea bag");
              println!("Pour water into a cup");
              println!("Let it steep");
          }
if we want to run these steps, we can run them at a go by `calling on the function "make_tea"`.

## Conditional Statements 
Conditional statements are code blocks that set the conditions for fulfilling a particular program. They are used to make decisions based on specific conditions. They allow the program to execute certain code blocks only when certain conditions are true, and to execute other code if those conditions are false.
In Rust, the main conditional statements are:
- **if Statement:** Used to execute code only `if` a condition is true.
  
          let number = 5;
          if number > 0 {
              println!("The number is positive.");
          }
- **if-else Statement:** Allows you to execute one block of code if the condition is true, and another block if it's false.
  
          let number = -3;
          if number > 0 {
              println!("The number is positive.");
          } else {
              println!("The number is negative.");
          }
- **else if Statement:** Used to check multiple conditions in sequence.
  
          let number = 0;
          if number > 0 {
              println!("The number is positive.");
          } else if number < 0 {
              println!("The number is negative.");
          } else {
              println!("The number is zero.");
          }
- **Conditional Expressions:** In Rust, if can also be used as an expression to assign values based on a condition.
  
          let condition = true;
          let number = if condition { 5 } else { 10 };
          println!("The number is: {}", number);
