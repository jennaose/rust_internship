enum Direction{
    North,
    South,
    East,
    West,
}

fn get_direction(direction: Direction)
{
    match direction{
        Direction::North => println!("you are going north"),
        Direction::South => println!("you are going south"),
        Direction::East => println!("you are going east"),
        Direction::West => println!("you are going west"),
    }
}

fn main (){
    let going = Direction :: West;// to construct the other variants just do the same thing you did with east
    get_direction(going);
}
