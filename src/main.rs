
const OUR_COURSE: &str = "Rust with AutoGPT";

fn main() {
    
    println!("Welcome to this course on {}!", OUR_COURSE);
    
    // Stack
    let x: i32;
    x = 2;
    println!("x is {}", x);

    let y: i32 = 4;
    println!("y is {}", y);

    // For loop
    for i in 0..=y {
        if i != 4 {
            print!("{}, ", i);
        } else {
            println!("{}", i);
        }
    }

    // Mutation
    let mut z: i32 = 5;
    print!("z was {} ", z);
    z = 10;
    println!("but is now {}", z);

    let freezing_temp: f64 = -2.4;
    println!("freezing_temp is {}", freezing_temp);

    let is_zero_remainder: bool = 10 % 4 != 0;
    println!("is_zero_remainder is {}", is_zero_remainder);

}
