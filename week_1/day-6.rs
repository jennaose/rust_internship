fn main (){
  
  //Lifetimes
    let string1= String :: from ("hello");
    let string2= String :: from ("worlds");
    let result = longest (&string1, &string2); //longest is the name of the function that takes two string references and returns the longest of the two.
    println!("the longest string is :{}", result);
}

fn longest<'a> (s1: &'a str, s2: & 'a str) ->& 'a str {
    if s1. len ()> s2. len(){
        s1
    } else {s2
    }
}
