fn main() {
    // ownership and scope
    let age = 33;
    println!("Age: {age}");

    {
        let is_handsome = true; // exists in the block
        println!("Is handsome: {is_handsome}");
    } // is_handsome goes out of scope here

    let age_two = 35; // pushed onto stack
    let is_handsome_two = true; // pushed onto stack

    println!("Age two: {age_two}"); // age_two used but still in stack
    println!("is handsome two: {is_handsome_two}"); // is_handsome_two used but still in stack

    copy_trait_example();
    string_type_example();

    string_manipulation();
    move_ownership();

    clone_method();
    borrowing_example();

    more_string_information();

    // ownership moves out of the bake_cake function and into the cake variable
    let cake = bake_cake();
    println!("Cake: {cake}");

    // ownership and function parameters
    let apples = 5;
    print_my_value(apples); // scalar type integer implements Copy trait so the value is copied and passed to the function
    println!("Apples is still valid: {apples}"); // apples is still valid

    let oranges = String::from("oranges"); // String does not implement the Copy trait
    print_my_value_string(oranges); // the value is moved and passed to the function, the original variable "oranges" is no longer valid

    let burger = String::from("burger"); // burger is not mutable but will get dropped when ownership moves
    add_fries(burger); // ownership of "burger" moves to the "meal" parameter in the function which IS mutable

    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
    current_meal = add_sugar(current_meal);
    println!("Current meal: {current_meal}");

    show_my_meal(&current_meal); // pass the current_meal as a reference

    let mut current_meal_new: String = String::new();
    add_flour_mutable_reference(&mut current_meal_new); // new current meal passed in as a mutable reference, is borrowed, can be modified
    add_sugar_mutable_reference(&mut current_meal_new);
    println!("Current meal new: {current_meal_new}");
    show_my_meal(&current_meal_new); // new current meal passed in as a reference, is borrowed

    ownership_with_immutable_and_mutable_references();

    car_mutable_immutable_references();

    // collection type ownership
    let registrations: [bool; 3] = [true, false, true]; // array of boolean values, owned by the array, which is owned by "registrations"
    let first = registrations[0]; // booleans implement the Copy trait so the value is copied and passed to the variable "first"
    println!("First: {first}, registrations: {registrations:?}"); // both are valid

    // array of Strings, owned by the array, which is owned by "languages"
    let languages: [String; 2] = [String::from("Rust"), String::from("Javascript")];
    // "languages" cannot partially own some of the elements, with other elements owned by other variables
    // let first_language = languages[0]; - this does not work, ownership of the value cannot be moved out of the array since it is still owned by languages
    // let first_language = languages[0].clone(); - this works, but is not an elegant solution because it fully copies the data in the heap
    let first_language = &languages[0]; // this is the ideal solution, as it borrows the value and does not take ownership, and is a pointer to the original value still
    println!("First: {first_language}, languages: {languages:?}"); // both are valid

    let languages_tuple = (String::from("Rust"), String::from("Javascript"));
    let first_language_tuple = &languages_tuple.0; // borrowing the memory address of the string in the tuple
    println!("First language tuple: {first_language_tuple}, Languages tuple: {languages_tuple:?}");
} // any in-scope function variables go out of scope here

fn copy_trait_example() {
    let time = 2025; // original value, integer type on stack
    let year = time; // value copied to a different variable, an independent value also on stack

    println!("Time: {time}");
    println!("Year: {year}");
}

fn string_type_example() {
    let food: &str = "pasta"; // string literal, exact value is known at compile time
    let food_two: String = String::from("spaghetti"); // string object, value can change at runtime

    println!("Food: {food}");
    println!("Food two: {food_two}");
}

fn string_manipulation() {
    let mut name = String::from("Josh");
    name.push_str(" is a developer");

    println!("{name}");
}

fn move_ownership() {
    let person = String::from("Josh"); // gets dropped the moment genius takes its reference/pointer to the heap
    let genius = person; // new variable get reference/pointer to original heap address, ownership of person is moved to genius, person is now invalid

    println!("{genius}");
}

fn clone_method() {
    let person = String::from("Josh");
    let genius = person.clone(); // creates a deep copy of the original value, both variables are valid

    println!("{genius}");
}

fn borrowing_example() {
    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value; // reference to my_stack_value, ownership is borrowed

    println!("my_integer_reference: {my_integer_reference}");

    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value; // reference to my_heap_value, ownership is borrowed

    println!("my_heap_reference: {my_heap_reference}");

    println!("my_integer_reference: {}", *my_integer_reference); // dereference the reference to grab the actual value
    println!("my_heap_reference: {}", *my_heap_reference); // dereference the reference to grab the actual value
}

fn more_string_information() {
    /*

      String - a dynamic piece of text stored on the heap

      &String (String reference) - a reference to a heap String

      str - a hardcoded, read-only piece of text encoded in the binary

      &str (str reference) - a reference to the text in memory that has loaded the binary file

    */

    let ice_cream = String::from("vanilla");
    let ice_cream_reference = &ice_cream;
    let ice_cream_literal = "strawberry";

    println!("Ice cream reference: {ice_cream_reference}");
    println!("Ice cream literal: {ice_cream_literal}");
}

// Ownership and function parameters
fn print_my_value(value: i32) {
    println!("Your value is: {value}");
}

fn print_my_value_string(value: String) {
    println!("Your value is: {value}");
}

// Mutable Parameters in functions, declared with the "mut" keyword before the parameter
fn add_fries(mut meal: String) {
    meal.push_str(" and fries");
    println!("Meal: {meal}");
}

// Function returning owned value to a calling function
fn bake_cake() -> String {
    String::from("strawberry shortcake")
}

// meal: String - the function takes ownership of the meal parameter but cannot modify it
// mut meal: String - the function takes ownership of the meal parameter and can modify it
// meal: &String -  the function borrows the meal parameter and cannot modify it, the parameter is now a reference
// meal: &mut String - the function borrows the meal parameter and can modify it, the parameter is now a mutable reference

fn add_flour(mut meal: String) -> String {
    meal.push_str("Add flour");
    meal
}

fn add_sugar(mut meal: String) -> String {
    meal.push_str("Add sugar");
    meal
}

fn show_my_meal(meal: &String) {
    println!("Meal steps: {meal}");
}

fn add_flour_mutable_reference(meal: &mut String) {
    meal.push_str("Add flour");
}

fn add_sugar_mutable_reference(meal: &mut String) {
    meal.push_str("Add sugar");
}

fn car_mutable_immutable_references() {
    // any number of immutable references allowed
    let car = String::from("Red"); // owner of the String in the heap
    let ref1 = &car; // immutable reference to String on the heap, an address, borrower, original "car" variable is still the owner and valid
    let ref2 = &car; // another immutable reference, exactly like ref1
    println!("{ref1} and {ref2} and {}", &car); // all valid, all immutable references

    // only one mutable reference allowed
    let mut mutable_car = String::from("Blue"); // owner of the String in the heap
    let new_ref1 = &mut mutable_car;
    // let new_ref2 = &mutable_car; - this would not be allowed because it is borrowed as a mutable above, and then used below
    // println!("{new_ref1} and {new_ref2}") - CANNOT use BOTH a mutable AND immutable reference at the same time
    println!("{new_ref1}"); // valid, the two above references can NOT co-exist - extends into a Rust concept called "lifetimes"
}

fn ownership_with_immutable_and_mutable_references() {
    let coffee = String::from("Mocha");
    let a = &coffee; // immutable reference, owner of its own reference
    let b = a; // immutable reference implements "Copy" trait, has its own reference data on the stack, so "a" remains valid with its own owned reference and so does "b"
    println!("{a} and {b} and {coffee}"); // all valid since any number of immutable references are allowed at any given existence

    let mut new_coffee = String::from("Latte"); // mutable variable
    let new_a = &mut new_coffee;
    let new_b = new_a; // mutable reference does NOT implement "Copy" trait, so new_b is NEW owner of reference, this is a "move", new_a is now invalid
                       // println!("{new_a} and {new_b} and {new_coffee}"); - does not work because new_a no longer exists, and new_coffee is borrowed so that cannot be used as well
                       // println!("{new_coffee}"); - this is a mutable borrow so it is invalid
    println!("{new_b}"); // new_b is the only valid reference here as it is the current owner of the mutable reference
    println!("{new_coffee}"); // new_coffee is valid here because the lifetime of the "borrow" is over after the final use of new_b above
}

// dangling reference error example
// fn create_city() -> &String {
//     let city = String::from("New York");
//     &city - this would not work because the reference would be dropped after the function ends (dies)
// }
// this function works instead because ownership of the value is being moved OUT of the function after the function ends (dies)
// fn create_city() -> String {
//     String::from("New York");
// }
