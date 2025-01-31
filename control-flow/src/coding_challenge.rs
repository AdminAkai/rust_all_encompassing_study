/*
Define a `color_to_number` function that accepts a 'color'
parameter (a string). Use if, else if, and else
statements to return a corresponding numeric value based
on the following rules:
1. If the color is "red", return 1.
2. If the color is "green", return 2.
3. If the color is "blue", return 3.
4. If the color is any other string, return 0.

Refactor the function above to use the `match` statement
instead of if, else if, and else.

Define a `factorial` function that calculates the
factorial of a number. The factorial is the product
of multiplying a number by every incremental
number leading up to it, starting from 1.

Examples:
The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
factorial(5) should return 120.

The factorial of 4 is 4 * 3 * 2 * 1 = 24
factorial(4) should return 24.

Implement two solutions/functions for the problem.
The first solution should not use recursion.
The second solution should use recursion.
*/

fn main() {
    let color_number = color_to_number("red");
    println!("Color number: {color_number}");

    let color_number_matcher = color_to_number_matcher("green");
    println!("Color number matcher: {color_number_matcher}");

    let factorial = factorial_function(5);
    println!("Factorial: {factorial}");

    let factorial_recursive = factorial_function_recursive(4);
    println!("Factorial recursive: {factorial_recursive}");
}

fn color_to_number(color: &str) -> i32 {
    if color == "red" {
        return 1;
    } else if color == "green" {
        return 2;
    } else if color == "blue" {
        return 3;
    } else {
        return 0;
    }
}

fn color_to_number_matcher(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial_function(number: i32) -> i32 {
    let mut baseline = 1;
    let mut result = 1;

    while baseline <= number {
        result *= baseline;
        baseline += 1;
    }

    result
}

fn factorial_function_recursive(number: i32) -> i32 {
    if number == 1 {
        return 1;
    }

    number * factorial_function_recursive(number - 1)
}
