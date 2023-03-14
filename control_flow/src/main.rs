fn main() {
    let number = 6;

    if number < 3 {
        println!("condition was right");
    } else {
        println!("condintion was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisble by 2");
    } else {
        println!("number is not divisble by 4, 3 or 2");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    loop_labels();
    while_loop();
    for_loop();
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is : {}", a[index]);

        index += 1;
    }
}

fn for_loop(){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in 1..4{
        println!("{number}");
    }

    for number in (1..=4).rev(){
        println!("{number}");
    }
}