struct Rectangle {
    width : u32,
    height : u32, 
}
impl Rectangle {
    fn area(&self)->u32{
        self.width*self.height
    }
    fn can_hold(&self, other: &Rectangle)->bool{
        self.width>other.width && self.height>other.height
    }
}
fn main(){
    let rect1 =Rectangle{width : 50, height: 30};
    let rect2= Rectangle {width: 30, height: 20};
    
    if rect1.can_hold (&rect2){
        println!("rect1 can hold rect2"); }

        else{ println!("rect1 can not hold rect2");
    }

    println! ("the area of rectangle 1 is: {}", rect1.area());
    println! ("the area of rectangle 2 is: {}", rect2.area());
    }
    /*the result will be rect1 can hold rect2
the area of rectangle 1 is: 1500
the area of rectangle 2 is: 600*/
