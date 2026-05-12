/**
 * if expressions
 *
 * if / else / else if
 */

fn main() {
    let age: i16 = 18;
    if age >= 18 {
        println!("You can drive a car");
    } else if age <= 21 {
        println!("You can't have a gun");
    } else {
        println!("You can not drive a car and a gun");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The number is: {number}");
}
