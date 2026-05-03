/**
 * Functions
 * Entry point
 * a function / variables should be written in snake_case
 * 
 * hoisting -> can call a function anywhere in your code
 * 
 * Expressions and Statements
 * Expression -> anything that returns a value
 * Statement -> anything that does NOT returns a value
 *              almost all statements in Rust end with ;
 * 
 * Expression
 * 5
 * true / false
 * add(3, 4)
 * if condition {value1} else {value2}
 * ({code})
 * 
 * Statement
 * let y = let x = 10;
 * 1 variable declaration: let x = 5;
 * 2 function definitions: fn foo() {}
 * 3 control flow statements: if    condition { // code }
 *                            else  condition { // code }
 *                            while condition { // code }
 */

fn main() {
    tell_height(182);
    tell_human("Victor", 27, 172.0);
    expression();
    statements();
}

fn tell_height(height: u32) {
    println!("The height is {}cm", height);
}

fn tell_human(name: &str, age: u32, height: f32) {
    println!("My name is {}. I'm {} yearls old and my height is {}cm", name, age, height);
}

fn expression() {
    let x = {
        let price: i32 = 5;
        let qty: i32 = 10;

        price * qty // the same as with return and ; 
    };

    println!("The result is: {}", x);

    let r = add(2, 2);
    println!("The add result is: {}", r);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn statements() {
    let weight: f32 = 70.0;
    let height: f32 = 1.70;
    let bmi = calculate_bmi(weight, height);
    println!("The BMI is: {:.2}", bmi);
}

fn calculate_bmi(weight_kg: f32, height_m: f32) -> f32 {
    weight_kg / (height_m * height_m)
}