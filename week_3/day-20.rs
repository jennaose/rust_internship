trait Describable {
    fn describe (&self)-> String;
}
struct Animal{
    specie:String,
    age:u32
}
impl Describable for Animal{
    fn describe (&self)-> String{
        format!("This {} is {} years old", self.specie, self.age)
      
    }
}
impl Animal {
fn update_age(&mut self) ->u32 {
    self.age +=1;//the idea was to just implement traits, but i decided to be creative and add this to test the knowlege of what i had learnt while creating my last inventory project
    self.age
}
}
fn main(){
    let mut animal= Animal {
        specie: ("Frog").to_string(),
        age: 30,
    };
println!("{}", animal.describe());
println!("he will be {} next year", animal.update_age());
}
