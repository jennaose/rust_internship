# Pattern Matching
Pattern matching in Rust allows you to check a value against different patterns and execute code depending on which pattern matches. Rust’s primary tool for pattern matching is the match statement.

## Match Expression
The match expression takes a value and matches it against a series of patterns. Here’s the syntax:

          match value {
              Pattern1 => code_to_run_if_pattern1_matches,
              Pattern2 => code_to_run_if_pattern2_matches,
              _ => code_to_run_if_no_patterns_match,
          }
The _ is a wildcard that matches anything, acting as a “catch-all” for cases that don’t match any specific pattern.

#### Example

            enum Direction {
                North,
                South,
                East,
                West,
            }
            
            fn travel(direction: Direction) {
                match direction {
                    Direction::North => println!("Heading North!"),
                    Direction::South => println!("Heading South!"),
                    Direction::East => println!("Heading East!"),
                    Direction::West => println!("Heading West!"),
                }
            }
            
**In this example:**

The match expression checks the value of direction.

It runs the corresponding code for each Direction variant, printing a message based on which direction is matched.
