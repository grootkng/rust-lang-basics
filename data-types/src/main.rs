/**
 * primitive data types
 * int, float, bool, char
 *
 * Integer
 * Rust has signed (+ and -) and unsigned intger (only +) types of different sizes.
 * signed integers: i8, i16, i32, i64, i128
 * unsigned integers: u8, u16, u32, u64, u128
 *
 * Floats (Floating Point Types)
 * f32, f64
 *
 * Boolean
 * true / false
 * 
 * Character Type / Char
 */

fn integers() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);
}

fn floats() {
    let pi: f32 = 3.1415;
    println!("Float f32: {}", pi);
}

fn booleans() {
    let is_raining: bool = true;
    println!("Boolean: {}", is_raining);
}

fn chars() {
    let letter: char = 'a';
    println!("Char: {}", letter);
}

fn main() {
    integers();
    floats();
    booleans();
    chars();
}
