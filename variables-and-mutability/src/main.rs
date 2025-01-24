// Constants
const TAX_RATE: f64 = 0.08;

// Type Alias
type Meters = i32;

fn main() {
  // Variables
  let apples = 50;
  let oranges = 14 + 6;
  let fruits = apples + oranges;

  println!("This year, my garden has {fruits} fruits.");

  // Mutable Variable
  let mut gym_reps = 10;
  println!("I plan to do {gym_reps} reps.");

  gym_reps = 5;
  println!("I plan to do {gym_reps} reps.");

  // Variable Shadowing
  let grams_of_protein = "100.345";
  println!("I ate {grams_of_protein} is a string.");

  let grams_of_protein = 100.345;
  println!("I ate {grams_of_protein} is f64.");

  let grams_of_protein = 100;
  println!("I ate {grams_of_protein} is i32.");

  // Scopes/Blocks
  let coffee_price = 5.99;

  {
    let cookie_price = 1.99;
    println!("The price of cookies is {cookie_price}.");
    println!("The price of coffee is {coffee_price}.");
  }

  // Constants Usage & Explicit Type Declaration
  let income: i32 = 100000;
  println!("The income is {income} and my tax rate is {TAX_RATE}.");

  // Type Alias Usage
  let mile_race_length: Meters = 1600;
  let two_mile_race_length: Meters = mile_race_length * 2;

  println!("A one mile race is {mile_race_length} meters long and a two mile race is {two_mile_race_length} long.");

  // Compiler Directives
  #[allow(unused_variables)]
  let unused_meters: Meters = 100;
}
