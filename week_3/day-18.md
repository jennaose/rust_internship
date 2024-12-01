### 1. **Iterators**

An **iterator** is any object in Rust that allows you to process over (or "iterate" over) a sequence of items, one by one. Rust’s iterators are **lazy** by default, meaning they don’t do any work until they’re consumed (for example, by a loop or a collecting function like `.collect()`).

- **Creating an Iterator**:
  - You can create an iterator from collections like vectors or hash maps with methods like `.iter()` or `.into_iter()`.
  ```rust
  let numbers = vec![1, 2, 3];
  let mut iter = numbers.iter(); // Creates an iterator over references to `numbers`
  ```

- **Using Iterators**:
  - **Consuming adapters**: Consumer iterators in Rust are methods that consume an iterator to produce a final result.  Once consumed, the iterator can no longer be used. Methods like `.collect()`, `.count()`, `.sum()`, and `.for_each()` consume the iterator and do something with each item. For instance:
    
    **sum():** Computes the sum of all items in the iterator.
    ```rust
let numbers = vec![1, 2, 3, 4, 5];
let total: i32 = numbers.iter().sum();
println!("{}", total); // 15
    ```
    
  - **Iterator adapters**: Functions like `.map()`, `.filter()`, and `.take()` create new iterators by transforming the items. They’re useful for chaining operations without changing the original data.
    ```rust
    let even_numbers: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even_numbers);
    ```

  The Iterator trait requires implementing the next method, which returns the next item in the sequence (or None if the sequence is finished).  
   ```rust
    let numbers = vec![1, 2, 3];

// Create an iterator from the vector
let mut iter = numbers.iter();

// Call `next` to fetch items one at a time
println!("{:?}", iter.next()); // Some(1)
println!("{:?}", iter.next()); // Some(2)
println!("{:?}", iter.next()); // Some(3)
println!("{:?}", iter.next()); // None (end of iteration)
```
Creating and Using Iterators
From Collections: Many collections like Vec, arrays, and HashMap have methods like .iter() or .into_iter() to create iterators.

rust
Copy code
let numbers = vec![1, 2, 3];
for num in numbers.iter() {
    println!("{}", num);
}
Using Ranges: Ranges like 1..5 are iterators.

rust
Copy code
for num in 1..5 {
    println!("{}", num); // Prints 1, 2, 3, 4
}
### 2. **Closures**

Closures in Rust are **anonymous functions** that can capture variables from the surrounding scope. They’re like functions but can "close over" the environment, which means they can use variables defined outside their body.

- **Basic Syntax**:
  - Closures are defined with pipes (`|`) for parameters, and they can infer types:
    ```rust
    let add_one = |x| x + 1;
    println!("Result: {}", add_one(5));
    ```
  - Closures can also take multiple parameters, and they don't need explicit type annotations unless necessary:
    ```rust
    let add = |a, b| a + b;
    println!("Sum: {}", add(2, 3));
    ```

- **Capturing Environment**:
  - Closures in Rust can **capture variables** by reference, by mutable reference, or by value:
    ```rust
    let num = 10;
    let add_num = |x| x + num; // `num` is captured by reference
    println!("Result: {}", add_num(5));
    ```
  
- **Using Closures with Iterators**:
  - Since closures can be passed as parameters, they’re often used with iterator methods like `.map()`, `.filter()`, etc.:
    ```rust
    let numbers = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("Squared numbers: {:?}", squared);
    ```

### Practical Example

Let’s combine iterators and closures in a more practical example. Suppose you have a list of numbers, and you want to filter only even numbers, square them, and collect the results into a new list.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    
    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)  // Filters only even numbers
        .map(|&x| x * x)           // Squares each number
        .collect();                // Collects the results into a Vec

    println!("Even squares: {:?}", even_squares);
}
```

- **Explanation**:
  - `.iter()` creates an iterator over `numbers`.
  - `.filter()` takes a closure `|&x| x % 2 == 0` to keep only even numbers.
  - `.map()` squares each remaining number.
  - `.collect()` gathers the transformed values into a new vector.
