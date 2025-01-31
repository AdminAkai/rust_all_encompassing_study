fn main() {
    let some_condition_that_we_cannot_predict_in_advance = true;

    if some_condition_that_we_cannot_predict_in_advance {
        println!("Hello, world!");
    }

    if false {
        println!("This will never be printed!")
    }

    let season = "summer";

    // if else-if and else chain
    if season == "summer" {
        println!("It's sunny outside!");
    } else if season == "winter" {
        println!("Brrm, it's cold outside!");
    } else if season == "fall" {
        println!("Leaves are falling!");
    } else if season == "spring" {
        println!("Flowers are blooming!");
    } else {
        println!("I don't know what season it is!");
    }

    even_or_odd(5);

    // match statement
    let evaluation = true;

    let value = match evaluation {
        true => 20,
        false => 40,
    };
    println!("The match value is: {value}");

    // match statement with multiple patterns, a default pattern, and a () return value
    match season {
        "summer" => println!("It's sunny outside!"),
        "winter" => println!("Brrm, it's cold outside!"),
        "fall" => println!("Leaves are falling!"),
        "spring" => println!("Flowers are blooming!"),
        _ => println!("I don't know what season it is!"),
    }

    // match statement with multiple values and conditionals
    let number = 8;

    match number {
        2 | 4 | 6 | 8 => println!("{number} is an even number"),
        1 | 3 | 5 => println!("{number} is an odd number"),
        _ => unreachable!(),
    }

    // match statement with nested if condition and evaluation saved in a variable used in an arm
    match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        value if value % 2 != 0 => println!("{value} is an odd number"),
        _ => println!("I don't know what number {number} is!"),
    }

    // loop statement
    let mut seconds = 10;

    loop {
        println!("{seconds} seconds left to blast off...");
        seconds -= 1;

        if seconds == 0 {
            println!("Blast off!");
            break;
        }
    }

    // loop statement with continue
    let mut timer = 30;

    loop {
        println!("{timer} seconds left to blast off...");
        if timer % 2 == 0 {
            println!("{timer} is an even number");
            timer -= 3;
            continue;
        }

        timer -= 1;

        if timer <= 0 {
            println!("Blast off!");
            break;
        }
    }

    // a while loop
    let mut seconds = 20;

    while seconds > 0 {
        println!("{seconds} seconds left to blast off...");
        seconds -= 1;
    }

    println!("Blast off!");
}

fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("{number} is an {result} number");
}
