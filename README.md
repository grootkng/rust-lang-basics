# Rust Concepts Explained

This document explains fundamental concepts demonstrated in the repository, using examples taken directly from the provided code.

## Primitive Data Types

Primitive data types are basic data types that do not have associated methods or complex structure.
 * Rust has signed (+ and -) and unsigned intger (only +) types of different sizes.
    * signed integers: i8, i16, i32, i64, i128
    * unsigned integers: u8, u16, u32, u64, u128
 * Floats (Floating Point Types)
    * f32, f64 
 * Boolean
    * true / false 
 * Character Type / Char

Example using `i32` and `f64`:
```rust
fn main() {
    let x: i32 = 5;
    let y: f64 = 150.55;
}
```

## Compound Data Types

Compound data types are structures used to group related data.

Example using array:
```rust
let nums: [i32; 5] = [1, 2, 3, 4, 5];
println!("Number Array: {:?}", nums);

let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
println!("Fruit's Array: {:?}", fruits);
```

Example using tuples:
```rust
let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
println!("Human: {:?}", human);

let mix = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
println!("Mix: {:?}", mix);
```

Example using slices:
```rust
let animals: &[&str] = &["Lion", "Elephant", "Crocodile"];
println!("Slice of animals: {:?}", animals);
```

Example using strings:
```rust
let mut stone_cold: String = String::from("Hell, ");
stone_cold.push_str("Yeah!");
println!("Stone cold says: {}", stone_cold);

let str: String = String::from("Hello, World");
let slice: &str = &str;
let slice_length: &str = &str[0..5];
println!("Slice: {}", slice);
println!("Slice length: {}", slice_length);
```

## Functions

Functions are reusable blocks of code that perform a specific task.

Example function:
```rust
fn check_balance(&self) {
    println!(
        "Account owned by {} has a balance of {}",
        self.owner, self.balance
    );
}
```

## Ownership, Borrowing and References

Ownership is the core concept in Rust for memory safety.

Example of Immutable Reference (`&`):
```rust
let r: &i32 = &x;
```

Example of Mutable Reference (`&mut`):
```rust
let mut y = 5;
let t = &mut y;
*t += 1;
```

## Variables and Mutability

Variables in Rust can be declared immutable or mutable.

Example of a Mutable Variable (`let mut`):
```rust
let mut account = BankAccount {
    owner: "Alice".to_string(),
    balance: 150.55,
};
```

## Constants

Needs to have its type explicity defined. Can also be used in global context.

```rust
const PI: f32 = 3.1415;

fn main() {
    constants();
}

fn constants() {
    const Y: i32 = 10;
    println!("The value of Y is: {}", Y);
    println!("The value of PI is: {}", PI);
}
```

## Shadowing

Shadowing allows you to declare a new variable with the same name, which is useful for changing the type of a variable.

```rust
let x = 5;
let x = 5 + 1;

{
    let x = x * 2;
    println!("The value of x in the inner scope is: {}", x);
}
println!("The value of x is: {}", x);
```

## Comments

Comments are used to explain the code and concepts.

Example of a Comment:
```rust
// immutable borrow to check the balance
```

## Control Flow

The `if`/`else` is pretty similar to any other language:
```rust
let age: i16 = 18;
if age >= 18 {
    println!("You can drive a car");
} else if age <= 21 {
    println!("You can't have a gun");
} else {
    println!("You can not drive a car and a gun");
}
```

It can be used as value:
```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

## Loops

In Rust exists `loop`, `while` and `for .. in ..`.  

Loops can be used as a value for a variable:
```rust
let mut counter: i16 = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

It can also have lables when having nested loops:
```rust
let mut count = 0;
'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
        println!("remaining = {remaining}");
        if remaining == 9 {
            break;
        }
        if count == 2 {
            break 'counting_up;
        }
        remaining -= 1;
    }

    count += 1;
}
```

The while loops:
```rust
let mut number = 3;
while number != 0 {
    println!("{number}!");

    number -= 1;
}
```

The for loops:
```rust
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("the value is: {element}");
}
```

## Structs

Structs are used to define custom data types.

Example of Struct Definition:
```rust
struct BankAccount {
    owner: String,
    balance: f64,
}
```

## Enums

Example of simple enum:
```rust
enum IpAddrKind {
    V4,
    V6,
}
```

Enum being used in a struct:
```rust
struct IpAddr {
    kind: IpAddrKind,
    address: String,
};

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};
```

Enums can have datatype with their values too:
```rust
enum IpAddrKind {
    V4(String),
    V6(String),
};

let home = IpAddrKind::V4(String::from("127.0.0.1"));
```

And they can be *enhanced* as well:
```rust
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
};

let home = IpAddrKind::V4(127, 0, 0, 1);
```

## Error handling

There's two ways to approach error handling in Rust.

The first one is with `Option<T>`:
```rust
enum Option<T> { // Define the generic Option type
    Some(T), // Represents a value
    None,    // Represents NO value
};
```

It can be use like:
```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

let result = divide(10.0, 0.0);
match result {
    Some(x) => println!("Result: {}", x),
    None => println!("Cannot divide by Zero"),
}
```

And the second approach with `Result<T, E>`:
```rust
enum Result<T, E> { // Define the generic Result type
    Ok(T),  // Represents a value
    Err(E), // Represents an error
};
```

And it can be used as:
```rust
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by Zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(err) => println!("Error: {}", err),
}
```