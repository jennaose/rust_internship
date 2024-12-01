fn main() {
    // Create a new vector
    let mut vec = Vec::new();

    // Add elements to the vector
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Access elements by index
    let first = vec[0];
    println!("First element: {}", first);

    // Iterate over the vector
    for i in &vec {
        println!("Element: {}", i);
    }

    // Remove the last element
    vec.pop();

    // Print the modified vector
    println!("Modified vector: {:?}", vec);
}
