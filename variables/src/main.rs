use std::io;
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;

    println!("{THREE_HOURS_IN_SECONDS}");

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";

    let spaces = spaces.len();

    println!("{spaces}");

    let mut a: i32 = 1;

    a = -1;

    println!("{a}");

    let b = 1_000;

    println!("{b}");

    let x = 2.9;

    let x: f32 = 3.0;

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 2;

    let remainder = 43 % 5;

    println!(
        "{} {} {} {} {} {}",
        sum, difference, product, quotient, truncated, remainder
    );

    let t = true;

    let f: bool = false;

    let c= 'z';

    let s = "z";

    let z: char = 'Z';

    let heart_eyed_cat = '😻';

    println!("{heart_eyed_cat}");

    let tup = (500, 6.3, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    let unit = ();

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // let a = [3, 3, 3, 3, 3, 3];

    let a = [a; 5];

    let a = [1,2,3,4,5];

    let first = a[0];

    let second = a[1];

    println!("{} {}", first, second);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
