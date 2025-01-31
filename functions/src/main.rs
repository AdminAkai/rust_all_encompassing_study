fn main() {
    // Functions and Arguments
    open_store("Atlanta");
    bake_pizza(5, "cheese");
    swim_in_profit();
    swim_in_profit();
    swim_in_profit();
    open_store("Brooklyn");
    bake_pizza(15, "pepperoni");

    // Functions with return values
    let squared_number = square(5);
    println!("Squared number: {squared_number}");

    let added_number = addition(5);
    println!("Added number: {added_number}");

    // Nested scope
    nested_scope();
}

fn open_store(neighborhood: &str) {
    println!("Opening pizza store in {neighborhood}.");
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizza(s).");
}

fn swim_in_profit() {
    println!("Swimming in profit.");
}

// Function with explicit return value
fn square(number: i32) -> i32 {
    return number * number;
}

// Function with implicit return value
fn addition(number: i32) -> i32 {
    number + number
}

// Function with a nested scope
fn nested_scope() {
    let multiplier = 5;

    let calculation = {
        let value = 5 + 4;
        value * multiplier // returned, similar to functions
    };

    println!("Calculation: {calculation}");
}
