use std::collections::HashMap;

fn main() {
    vectors();
    utf();
    hash_maps();
}

fn vectors() {
    let v: Vec<i32> = Vec::new();
    let mut the_vec = vec![1, 2, 3];
    the_vec.push(4);

    println!("The vector: {:?}", the_vec);

    let third = &the_vec[1];
    println!("The third element: {:?}", third);

    let third = the_vec.get(1);
    match third {
        Some(third) => println!("The third element with method get: {third}"),
        None => print!("Value not found at index 1"),
    }
}

fn utf() {
    let s = "foo".to_string();
    let s = String::from("foo");
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("The value of the string is: {s}");

    let s1 = String::from("hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    println!("The value of s3 is: {s3}");

    let hello = String::from("Hello,");
    let world = String::from("World!");
    let full_message = format!("{hello} {world}");
    println!("{full_message}");
}

fn hash_maps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The score of the team {} is {:?}", team_name, score);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
