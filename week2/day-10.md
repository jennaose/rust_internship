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

# Control Flow
Control flow is the order in which code executes, and it helps manage decisions and repetitive tasks in your code.

### if and else Statements

Use if to execute code based on a condition.
Example:

                    let number = 5;
                    if number > 0 {
                        println!("Positive number");
                    } else {
                        println!("Non-positive number");
                    }
### Match Expressions

match is also useful for control flow, not just for enums but for integers, booleans, ranges, and more.
Example:

                    let number = 3;
                    match number {
                        1 => println!("One"),
                        2 => println!("Two"),
                        3 => println!("Three"),
                        _ => println!("Something else"),
                    }
                    
### Loops (loop, while, and for)

**loop:** Repeats indefinitely until a break statement stops it.

                    let mut count = 0;
                    loop {
                        count += 1;
                        if count == 3 {
                            break;
                        }
                    }
                    
**while:** Continues running as long as a condition is true.

                    let mut number = 3;
                    while number != 0 {
                        println!("{}", number);
                        number -= 1;
                    }

**for:** Iterates over elements in a range or collection.

                    for i in 1..4 {
                        println!("{}", i);
                    }
                    
## Combining Pattern Matching and Control Flow
Pattern matching often complements control flow by enabling more readable and flexible code.

Example

                    fn number_category(num: i32) {
                        match num {
                            1 | 2 => println!("One or two"),
                            3..=5 => println!("Between three and five"),
                            _ => println!("Some other number"),
                        }
                    }
                    
Here, the match expression directs flow based on num, matching specific values or ranges to different actions.
