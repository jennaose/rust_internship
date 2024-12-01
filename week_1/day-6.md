# Ownership
Imagine you own a house. Since you're the owner, you control what happens with the house. If you move to a new house, you don't own the old house anymore. Someone else takes over as the owner. This is similar to how ownership in Rust works with values in memory.

**Ownership** is how rust controls who controls data in memory and Rust uses it to manage memory safely.
In Ownership each value in Rust has a variable that’s called its owner; one owner is allowed to a variable at a time and when the owner goes out of scope, the data is dropped i.e. if a variable owns some data and the variable stops being used, Rust automatically deletes that data to free memory.

Let's take it step-by-step with a very simple example:

Example 1: Ownership Moves

          fn main() {
              let s1 = String::from("hello");  // s1 is the owner of the string "hello"
              let s2 = s1;  // Ownership of the string is moved to s2
          
              println!("{}", s2);  // This works because s2 is the new owner
              // println!("{}", s1);  // Error! s1 is no longer valid because ownership was moved
          }

if you're working with owned strings in Rust (like String), the `String::from("hello") (or alternatively, "hello".to_string())` must be present when you want to create a heap-allocated, growable string (of type String).

If you simply use a string literal like "hello" (without String::from), Rust treats it as a string slice (&str), which is an immutable, fixed-length reference to a string. String slices (&str) are not growable, and they are stored in the binary of the program, not on the heap.

Rust uses this ownership system to ensure that:

- Memory is only cleaned up once: If s1 and s2 could both control the same string, both might try to free the same memory, causing errors.

- Memory leaks are avoided: When ownership ends (when a variable goes out of scope), Rust automatically frees the memory, so you don’t have to worry about it.

# Borrowing
You use this if You Want to Let Multiple People "Look" at the Data. Borrowing lets you let other parts of your code access the data without giving up ownership.

## Immutable Borrowing (&T): 
In Rust refers to temporarily borrowing data in a way that ensures the borrower can read the value but cannot modify it. Unlike mutable borrowing, multiple immutable borrows are allowed simultaneously, but you cannot have both an immutable and a mutable borrow at the same time.

Example

          fn main() {
              let s1 = String::from("hello");
          
              let len = calculate_length(&s1);  // We pass a reference to s1.
              println!("The length of '{}' is {}.", s1, len);  // s1 is still valid!
          }

          fn calculate_length(s: &String) -> usize {  // & means we're borrowing s1, not taking ownership
              s.len()  // We can access s, but we can't modify it
          }

**What happens here?**
- The `&` means immutable borrow:
&s1: This means “borrow s1.” Instead of moving ownership to calculate_length, we are lending it.
s1 remains the owner, but the function calculate_length can use the data temporarily.

- Inside calculate_length:  `s: &String means` "I can use s1, but I can’t change it (immutable reference)." `s.len()` gets the length of the string without changing its value.

- After calculate_length: `s1` is still valid because we only borrowed it. We didn't take ownership away from s1.

## Mutable Borrowing (&mut T): 
In Rust, mutable borrowing allows you to temporarily give access to a variable (or data) in a way that allows it to be modified by the borrower, while still retaining ownership of the data.

Important Rules of Mutable Borrowing:
Only one mutable reference: You can have only one mutable reference at a time, to avoid race conditions.

Example

          fn main() {
              let mut s1 = String::from("hello");  // s1 is initially "hello"
          
              change(&mut s1);  // s1 is borrowed mutably, so it can be modified in the function
          
              println!("{}", s1);  // After the modification, s1 is now "hello, world!"
          }

          fn change(s: &mut String) {
              s.push_str(", world!");  // This modifies the original string by appending ", world!"
          }

**What happens to s1?**
- Before the function call: `s1` starts out as "hello" because it's initialized with String::from("hello").
- Inside the change function: The `s1` variable is passed to the change function as a mutable reference using `&mut s1`.
Inside the function, `s.push_str(", world!")` appends the string ", world!" to s1. This means the content of s1 changes from "hello" to "hello, world!".
After the function call:

When you call println!("{}", s1); after the change function, s1 has been modified to "hello, world!", and that's what gets printed.

# Lifetimes
Lifetimes are a way for Rust to ensure that references are valid for as long as they need to be. They help prevent common programming errors, such as dangling references, which occur when you have a reference pointing to data that has been dropped.

## Why do we need lifetimes
- Memory Safety: Rust doesn’t have garbage collection, so it needs a way to manage memory safely. Lifetimes help ensure that references don’t outlive the data they point to.

- Ownership Rules: In Rust, each piece of data has an owner, and when the owner goes out of scope, the data is dropped. Lifetimes help the compiler understand how long references to that data should remain valid.

## Simple Example Without Lifetimes

Consider this simple example:

          fn main() {
              let s1 = String::from("Hello");  // s1 owns the string
              let r1 = &s1;                     // r1 is a reference to s1
          
              println!("{}", r1);               // This is fine because r1 is valid
          } // Here, s1 goes out of scope and is dropped

In this case, everything works fine because r1 is a reference to s1, which is still valid in that scope.

## What Happens Without Proper Lifetimes?
Now consider this example:

          fn main() {
              let r1; // r1 is declared but not initialized
              {
                  let s1 = String::from("Hello");
                  r1 = &s1; // r1 is a reference to s1
              } // s1 goes out of scope here
          
              println!("{}", r1); // Error: r1 is now a dangling reference
          }

After the inner scope ends, s1 is dropped, but r1 still tries to reference s1. This would cause a runtime error, so Rust prevents this at compile time.

## How Lifetimes Fix This
In Rust, you can use lifetime annotations to tell the compiler how long references should be valid. Here’s a simple function that uses lifetimes:

          fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
              if s1.len() > s2.len() {
                  s1 // Return reference to s1
              } else {
                  s2 // Return reference to s2
              }
          }

## Breakdown of the Function:
`<'a>:` This is a lifetime parameter. It tells the compiler that the references passed into the function `(s1 and s2)` must live at least as long as 'a.
`s1: &'a str:` This means `s1` is a reference to a string slice that lives at least as long as the lifetime `'a.`
The function returns a reference of the same lifetime `(&'a str)`, ensuring that the return reference will be valid as long as both `s1` and `s2` are valid.
