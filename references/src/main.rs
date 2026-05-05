/**
 * References and Borrowing
 * Safety and Performance
 * Borrowing and references are powerful concepts
 *
 * Understanding References
 * References: enable you to borrow values without taking ownership
 * Immutable reference
 * Mutable reference
 * Create reference by add "&"
 * -I- Immutable reference
 */

fn main() {
    reference();
    println!("");
    one_mutable_reference_or_many_imutable_references();
}

fn reference() {
    let x: i32 = 5;
    let r: &i32 = &x;
    println!("Value of x: {}", x);
    println!("Value of r: {}", r);

    let mut y = 5;
    let t = &mut y;
    *t += 1;
    println!("Value of t: {}", t);
}

fn one_mutable_reference_or_many_imutable_references() {
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };
    // immutable borrow to check the balance
    account.check_balance();

    // mutable borrow to withraw money
    account.withdraw(10.55);
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account owned by {}",
            amount, self.owner
        );
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "Account owned by {} has a balance of {}",
            self.owner, self.balance
        );
    }
}
