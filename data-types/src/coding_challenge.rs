/*
Declare an i32 variable assigned to 1337.
Use the underscore character to add a visual
separator between the numbers.

Cast the i32 to an i16 integer and assign the result
to a separate variable.

Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision.

Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean.

Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar.

Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar.

Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation.

Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/

fn main() {
    let value: i32 = 1337;
    let value_i16: i16 = value as i16;
    println!("Value: {value}");
    println!("Value i16: {value_i16}");

    let pi: f32 = 3.14159;
    println!("{pi:.3}");

    let with_milk: bool = true;
    let with_sugar: bool = true;

    let is_my_type_of_coffee: bool = with_milk && with_sugar;
    let is_acceptable_coffee: bool = with_milk || with_sugar;
    println!("Is my type of coffee: {is_my_type_of_coffee}");
    println!("Is acceptable coffee: {is_acceptable_coffee}");

    let array: [i8; 4] = [1, 2, 3, 4];
    println!("Array: {array:?}");

    let tuple: (i32, f32, bool, [i8; 4]) = (value, pi, is_my_type_of_coffee, array);
    println!("Tuple: {tuple:?}");
}
