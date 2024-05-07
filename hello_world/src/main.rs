enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let my_direction = Direction::East;
    let new_direction = my_direction;
    move_around(new_direction);
}

fn move_around(direction: Direction) {
    match direction {
        Direction::North => {
            println!("Moving North");
            // Implement North logic here
        }
        Direction::East => {
            println!("Moving East");
            // Implement East logic here
        }
        Direction::South => {
            println!("Moving South");
            // Implement South logic here
        }
        Direction::West => {
            println!("Moving West");
            // Implement West logic here
        }
    }
}
