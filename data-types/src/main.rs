fn main() {
    // Integers
    // Signed integer for 8-bit
    let eight_bit: i8 = -112;
    println!("{eight_bit}");

    // Unsigned integer for 8-bit
    let eight_bit_unsigned: u8 = 112;
    println!("{eight_bit_unsigned}");

    // Signed integer for 16-bit
    let sixteen_bit: i16 = -32500;
    println!("{sixteen_bit}");

    // Unsigned integer for 16-bit
    let sixteen_bit_unsigned: u16 = 64000;
    println!("{sixteen_bit_unsigned}");

    // Signed integer for 32-bit
    let thirty_two_bit: i32 = -2147483648;
    println!("{thirty_two_bit}");

    // Unsigned integer for 32-bit
    let thirty_two_bit_unsigned: u32 = 4294967295;
    println!("{thirty_two_bit_unsigned}");

    // Can also declare the type directly after the value, less readable
    let some_value = 20i8;
    println!("{some_value}");

    // usize and isize, also _ functions similarly to commas in regular mathematics
    let days: usize = 55;
    let years: isize = -15_000;

    println!("It's been {years} years and {days} days");

    // strings and special characters (escaped by a backslash)
    // new line character
    println!("Dear Emily, \nHow have you been?");
    // tab character
    println!("\tOnce upon a time");
    // double quote character
    println!("Juliet said \"I love you Romeo\"");

    // "escaping" backslashing backslashes so they can be used
    let filepath: &str = "C:\\My Documents\\new\\videos\\";
    println!("{filepath}");

    // can also use the "r" (raw) prefix on strings so Rust takes every character literally
    let filepath_new: &str = r"C:\My Documents\new\videos\";
    println!("{filepath_new}");

    // Methods
    // the "absolute" method on the i32 value
    // methods are invoked like functions
    let value: i32 = -15;
    println!("{}", value.abs());

    // string method "trim" removes white space from the beginning and end of a string
    let empty_space = "     my content     ";
    println!("{}", empty_space.trim());

    // integer method "pow" raises the integer to the power of the argument
    println!("{}", value.pow(2));
    println!("{}", value.pow(3));

    // floating point numbers
    let pi: f32 = 3.14159;
    println!("The current value of pi is {pi}");

    // floating point "floor" and "ceiling" methods for rounding down and up respectively
    println!("{}", pi.floor());
    println!("{}", pi.ceil());

    //floating point "round" method for rounding to the nearest whole number
    println!("{}", pi.round());

    // Format Specifier to customize the variable's visual representation
    // :.2 specifies that the value should be displayed with 2 decimal places
    println!("The value of pi is {pi:.2}");

    // also works outside of direct interpolation
    println!("The value of pi is {:.2}", pi);

    // Type Casting
    let miles_away = 50;
    let miles_away_i8 = miles_away as i8;
    println!("{miles_away}");
    println!("{miles_away_i8}");

    let float_miles_away = 100.329032;
    let float_miles_away_f32 = float_miles_away as f32;
    let float_miles_away_int = float_miles_away as i32;
    println!("{float_miles_away}");
    println!("{float_miles_away_f32}");
    println!("{float_miles_away_int}");

    // Math Operations
    let addition = 5 + 10;
    println!("Addition: {addition}");

    let subtraction = 10 - 6;
    println!("Subtraction: {subtraction}");

    let multiplication = 3 * 4;
    println!("Multiplication: {multiplication}");

    let floor_division = 5 / 3;
    println!("Floor Division: {floor_division}");

    let decimal_division = 5.0 / 3.0;
    println!("Decimal Division: {decimal_division}");

    let modulo = 10 % 3;
    println!("Modulo: {modulo}");

    // Augmented Assignment Operators
    let mut year = 2025;
    year += 1;
    println!("The new year is {year}");

    year -= 1;
    println!("The current year is {year}");

    year *= 2;
    println!("The year after doubling is {year}");

    year /= 4;
    println!("The year after dividing by 4 is {year}");

    // Boolean
    let is_handsome = true;
    let is_silly = false;
    println!("Handsome: {is_handsome}, Silly: {is_silly}");

    let age: i32 = 27;
    let is_young = age < 30;
    println!("Young: {is_young}");
    println!("{} {}", age.is_positive(), age.is_negative());

    let age: i32 = 40;
    let is_old = age > 30;
    let young = !is_old; // the "!" operator inverts the boolean
    println!("Old: {is_old}");
    println!("Young: {young}");

    // Equality and Inequality Operators
    println!("Coke is pepsi: {}", "Coke" == "Pepsi");
    println!("Coke is not pepsi: {}", "Coke" != "Pepsi");
    println!("Coke is the same as coke: {}", "Coke" == "coke");

    println!("5 is 5: {}", 5 == 5);
    println!("5 is not 5: {}", 5 != 5);

    println!("10.10 is 10.10: {}", 10.10 == 10.10);
    println!("10.10 is not 10.10: {}", 10.10 != 10.10);

    // Logic Operators
    // AND operator (&&)
    let purchased_ticket = true;
    let plane_on_time = true;
    let making_event = purchased_ticket && plane_on_time;
    println!("Making event: {making_event}");

    // OR operator (||)
    let user_has_paid_for_subscription = true;
    let user_is_admin = true;
    let premium_content_access = user_has_paid_for_subscription || user_is_admin;
    println!("Premium content access: {premium_content_access}");

    // Character
    let first_initial = 'J';
    println!("First initial: {first_initial}");

    println!("Is alphabetic: {}", first_initial.is_alphabetic()); // character method for determining if a character is a letter
    println!("Is uppercase: {}", first_initial.is_uppercase()); // character method for determining if a character is uppercase
    println!("Is lowercase: {}", first_initial.is_lowercase()); // character method for determining if a character is lowercase

    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let apples: [&str; 3] = ["Granny Smith", "Red Delicious", "Honeycrisp"];

    println!("Length: {}", numbers.len()); // using the len() method to get the length of the array
    println!("Length: {}", apples.len());

    let currency_rates: [f64; 0] = [];
    println!("Length: {}", currency_rates.len());

    let seasons: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];

    let first_season = seasons[0]; // accessing value at the array index
    let second_season = seasons[1];
    println!("First season: {first_season}, Second season: {second_season}");

    let mut mut_seasons: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];
    println!("Seasons: {}", mut_seasons[2]);
    mut_seasons[2] = "Autumn"; // changing array value via index
    println!("Seasons: {}", mut_seasons[2]);

    println!("Seasons: {seasons:#?}"); // using the Debug trait implemented on arrays to format the array into text

    // dbg! macro
    dbg!(2 + 2);

    // Tuples
    let employee = ("Molly", 32, "Marketing");
    // accessing tuple value by index
    // let name = employee.0;
    // let age = employee.1;
    // let department = employee.2;

    let (name, age, department) = employee; // destructuring the tuple into individual variables

    println!("Name: {name}, Age: {age}, Department: {department}");
    println!("Employee: {employee:#?}"); // using the Debug trait implemented on tuples to format the tuple into text

    // Ranges
    let month_days: std::ops::Range<i32> = 1..31;
    let month_days_inclusive: std::ops::RangeInclusive<i32> = 1..=31;
    println!("{month_days:?}");
    println!("{month_days_inclusive:?}");

    // using the "for" loop to iterate over a range of integers
    for number in month_days {
        println!("{number}");
    }

    // a range also works with characters
    let letters: std::ops::Range<char> = 'b'..'f';

    // using the "for" loop to iterate over a range of characters
    for letter in letters {
        println!("{letter}");
    }

    // arrays can also be used in a "for" loop
    let colors = ["Red", "Green", "Blue", "Yellow", "Purple"];

    // using the "for" loop to iterate over an array
    for color in colors {
        println!("{color}");
    }
}
