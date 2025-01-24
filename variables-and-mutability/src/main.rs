// Constants
const TAX_RATE: f64 = 0.08;

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

}
