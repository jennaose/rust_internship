fn main (){
//Ownership in Rust
    let s1= String:: from ("hello");
    let s2 =s1;// ownership is hello has been transferred from s1 to s2
    println!("{}", s2);

}


fn main (){
    //Immutable borrowing in Rust
    let s1 = String :: from ("hello");
    let len = calculate_length (&s1); // We pass a reference to s1
    println!("the length of '{}' is {}", s1, len);  //because s1 is still valid!

    fn calculate_length (s: &String ) -> usize// & means we're borrowing s1, not taking ownership
     { s.len()
    }
}


fn main() {
    //mutable borrowing in Rust
    let mut s1 = String::from("hello");  // s1 is initially "hello"

    change(&mut s1);  // s1 is borrowed mutably, so it can be modified in the function

    println!("{}", s1);  // After the modification, s1 is now "hello, world!"
}

fn change(s: &mut String) {
    s.push_str(", world!");  // This modifies the original string by appending ", world!"
}


fn main (){
    //lifetimes in Rust
    let string1= String :: from ("hello");
    let string2= String :: from ("worlds");
    let result = longest (&string1, &string2); //longest is the name of the function that takes two string references and returns the longest of the two.
    println!("the longest string is :{}", result);
}

fn longest<'a> (s1: &'a str, s2: & 'a str) ->& 'a str {
    if s1. chars().count()< s2.chars().count()
    { s1
    } else {s2
    }
}
