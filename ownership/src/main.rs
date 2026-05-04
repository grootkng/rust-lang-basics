/**
 * Ownership, Borrowing and References
 *
 * Ownership
 * C / C++ -> Memory Management Control Issue
 * Garbage Collector solved this issue, but created a new issue
 * [stopping/resuming the program]
 * Ownership intruced by rust to solve memory safety issues and high performance at the same time.
 *
 * What is Ownership?
 * Every value has a single owner [every variable has one value and it is its sole owner]
 *
 * Ownership Rules
 * 1 - Each value in Rust has a variable that's its onwer.
 * 2 - There can be only one onwer at a time.
 * 3 - When the onwer goes out of scope the value will be dropped.
 *
 */

fn first_rule() {
    let s1 = String::from("RUST");
    let len = first_rule_calculate_length(&s1);
    println!("[Fist rule] Length: {}", len);
}

fn first_rule_calculate_length(s: &String) -> usize {
    s.len()
}

fn second_rule() {
    let s1 = String::from("RUST");
    let s2 = s1;
    println!("[Second rule] String: {}", s2); // the value "RUST" was moved to s2 and now s2 is the owner of that value
}

fn third_rule() {
    let s1 = String::from("RUST");
    let len = first_rule_calculate_length(&s1);
    println!("[Fist rule] Length: {}", len);
}

fn third_rule_calculate_length(s: &String) -> usize {
    // println!("{}", s1); // a variable only exists in a scope
    s.len()
}

fn main() {
    first_rule();
    second_rule();
    third_rule();
}
