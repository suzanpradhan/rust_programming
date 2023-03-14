fn main() {
    println!("Hello, world!");

    another_function(232);

    printed_labeled_measurementt(5, 'h');

    exp_and_state();

    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(1);
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("Another Function!");

    println!("The value of x is: {x}");
}

fn printed_labeled_measurementt(value: i32, unit_label: char) {
    print!("The measurement is: {value}{unit_label}");
}

fn exp_and_state() {
    let y = 6;

    //- let x = (let y = 6);
    let x = {
        let y = 6; // statement
        y //expression
    };
    println!("The value of x is: {x}");

    let y = {
        let x = 3; //statement
        x + 1 //expression
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5 // return keyword can also be used
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
