use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Adding elements
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    // Accessing values
    let team_name = "Blue";
    match scores.get(team_name) {
        Some(score) => println!("The score for {} is {}", team_name, score),
        None => println!("No score found for {}", team_name),
    }

    // Updating values
    scores.insert("Blue", 25); // This updates the score for "Blue"

    // Iterating over the hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Removing an element
    scores.remove("Blue");

    // Display the final hash map
    println!("Final scores: {:?}", scores);
}

