fn main() {
    let action_hero = String::from("Jackie Chan"); // String data in the heap, address on the stack
    let string_reference = &action_hero; // create string reference
    println!("{string_reference} is an action hero.");

    let first_name = &action_hero[0..6]; // reference to the first 6 byte indexes of the string, a string slice
    println!("First name: {first_name}");

    let last_name = &action_hero[7..11]; // reference to the last 4 byte indexes of the string, a string slice
    println!("Last name: {last_name}");

    let other_action_hero = "Jet Li"; // string literal, a reference to the embedded data in the executable itself
    let other_first_name = &other_action_hero[0..3]; //  identifies the address of the data in the executable, then creates its OWN independent reference to it
    println!("First name: {other_first_name}"); // this will be valid even if other_action_hero is dropped because it is its own independent reference

    let other_last_name = &other_action_hero[4..6]; // same thing as above
    println!("Last name: {other_last_name}");

    let food = "pizza"; // 5 bytes, one english character occupies one byte in memory
    println!("Food: {}", food.len()); // length of the string in bytes

    let pizza_slice = &food[0..3]; // reference to the first 3 bytes of the text data in the executable
    println!("Pizza slice: {}", pizza_slice.len()); // length of the string slice in bytes

    let new_action_hero = String::from("Bruce Lee");
    let new_first_name = &new_action_hero[..5]; // syntactic shortcut for ranges, if it starts at 0 the 0 can be omitted
    println!("First name: {new_first_name}");

    let last_name = &new_action_hero[6..]; // syntactic shortcut for ranges, if it ends at the end of the string the end can be omitted
    println!("Last name: {last_name}");

    let full_name = &new_action_hero[..]; // syntactic shortcut for ranges, if both numbers are omitted then the range is from the beginning to the end of the string
    println!("Full name: {full_name}");

    do_hero_stuff(&action_hero);
    // do_hero_stuff(other_action_hero); // this will not work because the function expects a reference to a String type, not a string literal

    // now a function with the opposite parameter type
    do_hero_stuff_updated(other_action_hero);
    do_hero_stuff_updated(&action_hero); // &String still works here because Rust has a feature called deref coercion which de-references the String reference and converts it to a &str type behind the scenes

    let values = [4, 8, 15, 16, 23, 42]; // array of integers
    let my_slice = &values[0..3]; // borrowing the first 3 elements of the array (index 3 is not included)
    println!("Slice: {:?}", my_slice); // print the slice

    let my_slice_two = &values[..4]; // borrowing the first 4 elements of the array (index 4 is not included)
    let my_slice_three = &values[2..]; // borrowing the last 4 elements of the array (index 2 is included)
    let my_slice_four = &values[..]; // borrowing the entire array
    println!("Slice two: {:?}", my_slice_two);
    println!("Slice three: {:?}", my_slice_three);
    println!("Slice four: {:?}", my_slice_four);

    let regular_array_reference = &values; // reference to the entire array
    print_length(regular_array_reference); // pass the reference to the function
    let array_slice_of_three = &values[..3]; // reference to the first 3 elements of the array, no concrete length is specified in the inferred type
                                             // print_length(array_slice_of_three); - does not work because of a type difference and array slice cannot be coerced into an array reference with a specific concrete length
    print_length_new(array_slice_of_three); // this works because the types are the same
    print_length_new(regular_array_reference); // this works because the array reference is being deref coerced into an array slice reference

    let mut my_array = [10, 15, 20, 25, 30]; // mutable array
    println!("Mutable array: {:?}", my_array);
    let my_mutable_slice = &mut my_array[2..4]; // mutable slice of an array, mutable reference to my_array, array is borrowed
    println!("Mutable slice: {:?}", my_mutable_slice);

    my_mutable_slice[0] = 100; // even though this is a slice, this mutates the original array, since my slice points to the original array
    println!("Mutated slice: {:?}", my_mutable_slice); // prints the slice, borrow is finished
    println!("Mutated array: {:?}", my_array); // prints the original array, which has been mutated
}

fn do_hero_stuff(hero_name: &String) {
    println!("{hero_name} saves the day!.");
}

fn do_hero_stuff_updated(hero_name: &str) {
    println!("{hero_name} saves the day!.");
}

// function that takes a reference with a specific concrete length
fn print_length(reference: &[i32; 6]) {
    println!("{:?}", reference.len());
}

// similar function but takes an array slice rather than a reference with a specific concrete length
fn print_length_new(reference: &[i32]) {
    println!("{:?}", reference.len());
}
