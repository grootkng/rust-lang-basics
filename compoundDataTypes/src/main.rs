/**
 * Compound Data Types
 * arrays, tuples, slices and strings (slice string)
 *
 * Arrays
 * items needs to be of the same type
 *
 * Tuples
 * items can be of different types
 *
 * Slices
 *
 * String vs String Slices (&str)
 * Strings -> growable, mutable, owned string type -> Heap  (slow)
 * String Slice -> fixed size                      -> Stack (fast) 
 *
 */

fn arr() {
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", nums);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruit's Array: {:?}", fruits);
    println!("Fruit's array first element: {:?}", fruits[0]);
}

fn tuples() {
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human: {:?}", human);

    let mix = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("Mix: {:?}", mix);
}

fn slices() {
    let nums: &[i32] = &[1, 2, 3, 4, 5];
    println!("Slice of nums: {:?}", nums);

    let animals: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Slice of animals: {:?}", animals);

    let books: &[&String] = &[
        &"IT".to_string(),
        &"Harry Potter".to_string(),
        &"ZEN".to_string(),
    ];
    println!("Slice of books: {:?}", books);
}

fn strings() {
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone cold says: {}", stone_cold);

    let str: String = String::from("Hello, World");
    let slice: &str = &str;
    let slice_length: &str = &str[0..5];
    println!("Slice: {}", slice);
    println!("Slice length: {}", slice_length);
}

fn main() {
    arr();
    tuples();
    slices();
    strings();
}
