fn main() {
    fn approach_1() {
        // approach 1
        enum Option<T> {
            // Define the generic Option type
            Some(T), // Represents a value
            None,    // Represents NO value
        };
    }

    fn divide_approach_1(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }

    let result = divide_approach_1(10.0, 0.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by Zero"),
    }

    fn approach_2() {
        // approach 2
        enum Result<T, E> {
            // Define the generic Result type
            Ok(T),  // Represents a value
            Err(E), // Represents an error
        };
    }

    fn divide_approach_2(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err("Cannot divide by Zero".to_string())
        } else {
            Ok(numerator / denominator)
        }
    }

    match divide_approach_2(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
