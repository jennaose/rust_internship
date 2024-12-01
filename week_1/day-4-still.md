# Loops
In Rust, there are three main types of loops: loop, while, and for. Each loop serves different purposes depending on how you want to control the flow of repetition. Let's break down each type of loop.
1. **loop (Infinite Loop)** The loop keyword creates an infinite loop, which will continue to run forever unless explicitly stopped with a break statement. It's useful when you need to repeatedly execute code without a predetermined end condition.

Example:

              fn main() {
              let mut counter = 0;
              loop {
                  counter += 1;
                  println!("Counter: {}", counter);
          
                  if counter == 5 {
                      break; // Exits the loop when counter reaches 5
                  }
              }
              println!("Loop ended");
          }
   
•	Explanation: The loop will continue indefinitely until the break condition is met. In this example, the loop terminates when counter reaches 5.

2. **while Loop** The while loop repeats as long as a specified condition evaluates to true. It's useful when you want the loop to stop once a condition is no longer met.

Example:

          fn main() {
              let mut number = 3;
          
              while number != 0 {
                  println!("Number is: {}", number);
                  number -= 1; // Decrease the number each time
              }
          
              println!("Reached zero!");
          }
   
•	Explanation: The while loop checks the condition (number != 0) before each iteration. The loop runs until the condition becomes false (i.e., when number becomes 0).

3. **for Loop:** The for loop is used to iterate over a range or a collection (like arrays or vectors). It's very common in Rust for iterating through elements of a collection or performing an action a specific number of times.

Example 1: Iterating Over a Range

          fn main() {
              for number in 1..5 {
                  println!("Number: {}", number);
              }
              println!("Done!");
          }
          
•	Explanation: The for loop iterates over the range 1..5 (which includes 1, 2, 3, and 4, but not 5). The loop runs once for each value in the range.

Example 2: Iterating Over a Collection

          fn main() {
              let numbers = [10, 20, 30, 40];
          
              for num in numbers.iter() {
                  println!("Number: {}", num);
              }
          }
          
•	Explanation: The for loop iterates over each element in the numbers array and prints it.
•	In Rust, iter is a method that creates an iterator over the elements of a collection, such as an array, vector, or other iterable types.

4. **Loop Control:** break and continue

•	break: Used to exit a loop prematurely, even if the loop's condition is still true.

•	continue: Skips the rest of the current loop iteration and moves to the next one.

Example Using break and continue:

          fn main() {
              for number in 1..10 {
                  if number == 5 {
                      println!("Breaking at 5!");
                      break; // Exits the loop
                  }
                  if number % 2 == 0 {
                      continue; // Skips even numbers
                  }
                  println!("Odd number: {}", number);
              }
          }
   
•	Explanation:

**break** is used to exit the loop when number reaches 5.
 
**continue** skips the current iteration when number is even, so only odd numbers are printed.
