# Rust Concepts Explained

## Table of Contents

1. [Primitive Data Types](#primitive-data-types)
2. [Compound Data Types](#compound-data-types)
3. [Functions](#functions)
4. [Ownership, Borrowing and References](#ownership-borrowing-and-references)
5. [Variables and Mutability](#variables-and-mutability)
6. [Constants](#constants)
7. [Shadowing](#shadowing)
8. [Comments](#comments)
9. [Control Flow](#control-flow)
10. [Loops](#loops)
11. [Structs](#structs)
12. [Enums](#enums)
13. [Error handling](#error-handling)
13. [Collection types](#collection-types)

---

## Primitive Data Types

Basic types without complex structure.

* **Integers**: `i8`..`i128` (signed), `u8`..`u128` (unsigned).
* **Floats**: `f32`, `f64`.
* **Boolean**: `true` / `false`.
* **Character**: `char`.

```rust
fn main() {
    let x: i32 = 5;
    let y: f64 = 150.55;
}
```

## Compound Data Types

Structures grouping related data.

**Array** (Fixed length):

```rust
let nums: [i32; 5] = [1, 2, 3, 4, 5];
```

**Tuples** (Different types):

```rust
let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
```

**Slices** (References to sequence):

```rust
let animals: &[&str] = &["Lion", "Elephant"];
```

**Strings**:

```rust
let mut stone_cold = String::from("Hell, ");
stone_cold.push_str("Yeah!");
```

## Functions

Reusable blocks of code.

```rust
fn check_balance(&self) {
    println!("Owner: {} | Balance: {}", self.owner, self.balance);
}
```

## Ownership, Borrowing and References

Memory safety via ownership rules.

**Immutable Reference (`&`):**

```rust
let r: &i32 = &x;
```

**Mutable Reference (`&mut`):**

```rust
let mut y = 5;
let t = &mut y;
*t += 1;
```

## Variables and Mutability

Variables are immutable by default.

```rust
let mut account = BankAccount {
    owner: "Alice".to_string(),
    balance: 150.55,
};
```

## Constants

Global/Local scope, explicit type required.

```rust
const PI: f32 = 3.1415;
```

## Shadowing

Re-declaring variable with same name to change value or type.

```rust
let x = 5;
let x = x + 1;
```

## Comments

```rust
// line comment
```

## Control Flow

```rust
if age >= 18 {
    println!("You can drive a car");
} else if age <= 21 {
    println!("You can't have a gun");
} else {
    println!("Wait");
}
```

## Loops

`loop`, `while` and `for`.

```rust
for element in [10, 20, 30] {
    println!("{element}");
}
```

## Structs

Custom data types.

```rust
struct BankAccount {
    owner: String,
    balance: f64,
}
```

## Enums

Variants of a type.

```rust
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

## Error handling

Safe alternatives to `null` and exceptions.

**Option** (Presence/Absence):

```rust
match divide(10.0, 0.0) {
    Some(x) => println!("{x}"),
    None => println!("Zero div"),
}
```

**Result<T, E>** (Success/Failure):

```rust
match divide(10.0, 2.0) {
    Ok(res) => println!("{res}"),
    Err(e) => println!("{e}"),
}
```

## Collection Types

### Vectors

Store values of same type in continued memory. Dynamic.

```rust
// Initialization and mutability
let mut v = vec![1, 2, 3];
v.push(4);

// Access by index (can cause panic if out of band)
let second = &v[1]; 

// Acces with .get() (safe, returns Option)
match v.get(1) {
    Some(val) => println!("Valor: {val}"),
    None => println!("Índice inexistente"),
}

// Loop
for x in &v {
    println!("{x}");
}

```

### UTF-8 Strings

Grow dinamically. Collection of interpreted bytes as UTF-80 characteres.

```rust
let mut s = String::from("foo");
s.push_str("bar"); // Adiciona slice
s.push('!');       // Adiciona char

// Eficient concat
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 has been moved here and can no longer be used

// Format (don't move the variables)
let full = format!("{s3} Welcome");

```

### Hash Maps

Store key-value pairs. Handful for searching identifiers.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Safe access with fallback
let team = String::from("Blue");
let score = scores.get(&team).copied().unwrap_or(0);

// Conditional insertion (insert only if not exists)
scores.entry(String::from("Red")).or_insert(30);

for (key, value) in &scores {
    println!("{key}: {value}");
}

```