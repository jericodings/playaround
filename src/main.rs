
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

    let my_char: char = 'z';
    println!("my_char is {}", my_char);

    let emoji_char: char = '😎';
    println!("emoji_char is {}", emoji_char);

    let my_floats: [f32; 10] = [0.0; 10];
    println!("my_floats is {:?}", my_floats);

    let my_floats_new: [f32; 10] = my_floats.map(|n| n + 2.0);
    println!("my_floats_new is {:?}", my_floats_new);

    let name: &str = "Jeric";
    println!("name is {:?}", name);

    let dynamic_name: String = String::from("Jeric Codings");
    println!("dynamic_name is {:?}", dynamic_name);
    println!("my dynamic_name stored in memory {:p}", &dynamic_name);

    let str_slice: &str = &dynamic_name[0..5];
    println!("str_slice is {:?}", str_slice);

    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'h');
    chars.insert(1, 'e');
    chars.insert(2, 'l');
    chars.push('l');
    chars.push('o');
    chars.push('.');
    println!("chars is {:?}", chars);
    dbg!(&chars);

    let removed_char: char = chars.pop().unwrap();
    println!("removed_char is {:?}", removed_char);
    println!("chars is {:?}", chars);

    // chars.iter().for_each(|c| print!("{}", c));

    let chars_again: Vec<char> = vec!('h','e','l','l','o');
    dbg!(&chars_again);

    let collected: String = chars_again.iter().collect();
    dbg!(collected);

    for c in chars_again {
        print!("{}", c);
        if c == 'o' {
            println!(", world!");
        }
    }

    // Closures
    
    let num: i32 = 5;
    let add_num = |x: i32| x + num;
    let new_num: i32 = add_num(7);
    dbg!(new_num);

}
