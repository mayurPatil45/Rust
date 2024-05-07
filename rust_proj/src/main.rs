fn main() {
    // println!("Hello, world!");

    // Vars
    // let x = 5;
    // let x: i32 = -5;
    // let y: u32 = 69;
    // let z: f32 = 69.69;
    // println!("x: {}, y: {}, z: {}", x, y, z);



    // Boolean
    // let is_male = false;
    // let is_above_18: bool = true;

    // if is_male{
    //     println!("You are Male");
    // }
    // else {
    //     println!("You are not a male");
    // }

    // if is_male && is_above_18{
    //     print!("You are a Legal Male");
    // }



    // String
    // let greeting = String::from("Hello World");
    // println!("{},", greeting);

    // println!("{}", greeting.chars().nth(1000))

    // let guess: String = "42".parse().expect("Not a number!");
    // let x= "5";
    // let y = guess + x;
    // print!("{}", y)


    
    // Loops
    // for i in 0..11 {
    //     println!("Hello World, {}!!", i);
    // }


    
    // Functions
    // let a = 10;
    // let b = 30;
    // let sum = do_sum(a, b);
    // println!("Sum of {} and {} is {}", a, b, sum);



    // Memory managment in rust
    // has its own ownership model its like c means manual but there are some rules so that there are no dangling pointer issues or any other memory issue for that matter

    // Not having a garbage collector is one of the key reasons rust is so fast
    // It achieves this using the
    // Mutability
    // Heap and memory
    // Ownership model
    // Borrowing and references
    // Lifetimes



    // Mutability - by default all the vars in rust are immutable. 
    // Immutable data is inherently thread-safe because if no thread can alter the data, then no synchronization is needed when data is accessed concurrently.
    // Knowing that certain data will not change allows the compiler to optimize the code better.
    // let mut x = 32;
    // x = 2;
    // println!("{}", x);



    // Stack and Heap
    // Stack: Fast allocation and deallocation. Rust uses the stack for the most primitive data types and for the data where the size is known at the compile time(eg. Numbers).
    // Heap: Used for the data that can grow at runtime, such as vectors or strings.



    // Ownership
    // Ownership is set of rules that govern how a Rust program manages memory.
    // Keeping track of what parts of code are using what data on the heap,minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses


    // Structs
    // struct User{
    //     active: bool,
    //     username: String,
    //     email: String,
    //     sign_in_count: u64,
    // }

    // fn main(){
    //     let user1 = User {
    //         active: true,
    //         username: String::from("someusername123"),
    //         email: String::from("someone@example.com"),
    //         sign_in_count: 1,
    //     };
    //     print("User 1 username {:?}", user1.username);
    // }



    // Implement Struct
    // struct Rect{
    //     width: u32,
    //     height: u32,
    // }

    // impl Rect {
    //     fn area(&self) -> u32 {
    //         self.width * self.height
    //     }
    // }

    // fn main(){
    //     let rect = Rect {
    //         width: 30,
    //         height: 30,
    //     };
    //     print!("The area of rectangle is {}", rect.area());
    // }



    // enums - makes code more stricker as you can play with only defined values
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
    

}


// // fn do_sum(a: i32, b: i32) -> i32 {
// //     return a + b;
// // }

// fn main() {
//     // Create a string "Hello" and bind it to s1
//     let s1 = String::from("Hello");

//     // At this point, s1 has ownership of the string "Hello"

//     // Get the memory address of the string "Hello" owned by s1
//     let address_of_s1 = s1.as_ptr() as usize;

//     // Transfer ownership from s1 to s2
//     let s2 = s1;

//     // Now s2 owns the string "Hello", and s1 is no longer valid

//     // Get the memory address of the string "Hello" owned by s2
//     let address_of_s2 = s2.as_ptr() as usize;

//     // Print the memory addresses
//     println!("Memory address of s1's string: {:p}", address_of_s1 as *const ());
//     println!("Memory address of s2's string: {:p}", address_of_s2 as *const ());
// }
 
