# Rust Concepts Explained

This document explains fundamental concepts demonstrated in the repository, using examples taken directly from the provided code.

## Primitive Data Types

Primitive data types are basic data types that do not have associated methods or complex structure.

Example using `i32` and `f64`:
```rust
fn main() {
    let x: i32 = 5;
    let y: f64 = 150.55;
}
```

## Compound Data Types

Compound data types are structures used to group related data.

Example using a Struct:
```rust
struct BankAccount {
    owner: String,
    balance: f64,
}
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