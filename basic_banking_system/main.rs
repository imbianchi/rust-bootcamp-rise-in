// create Accoaunt trait to define methods
trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> Result<f64, String>;
}
// BankAccount struct
struct BankAccount<'a> {
    account_number: u32,
    holder_name: &'a str,
    balance: f64,
}

// implement the methods logic
impl Account for BankAccount<'_> {
    // deposit amount function
    // @param amount f64
    // must be positive number
    // returns a Result with unit or Err string
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        self.balance += amount;
        Ok(())
    }

    // withdraw amount function
    // @param amount u32
    // must be positive number
    // must have amount available grater or equal of the balance
    // returns a Result
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        // check if the user have the amount available
        // if not, returns a error with a message
        if self.balance < amount {
            return Err("Not enough money to withdraw!".to_string());
        }

        // if the user have the amount to withdraw
        // subtract the amount of balance
        self.balance -= amount;
        Ok(())
    }

    // balance function
    // returns a Result with the amount of user's balance f64 or Err
    fn balance(&self) -> Result<f64, String> {
        Ok(self.balance)
    }
}

fn main() {
    // create a bank account instance number one - module 3
    let mut account_number_one = BankAccount {
        account_number: 1,
        holder_name: "The Dude with the First Account",
        balance: 0.0,
    };

    let deposit_amount = 100.00;
    let withdraw_amount = 50.0;

    // Deposit a $hundred into bank account one
    match account_number_one.deposit(deposit_amount) {
        Ok(()) => println!("Deposit amount done with success!"),
        Err(message) => println!("Error: {}.", message),
    }

    // Withdraw a $hundred from bank account one
    match account_number_one.withdraw(withdraw_amount) {
        Ok(()) => println!("Withdraw amount done with success!"),
        Err(e) => println!("Error: {}.", e),
    }

    // Getting balance from account number one
    match account_number_one.balance() {
        Ok(value) => println!("Your total balance value is: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // Withdraw more money that the account have (forced error)
    match account_number_one.withdraw(withdraw_amount * 2.0) {
        Ok(()) => println!("Withdraw amount done with success!"),
        Err(e) => println!("Error: {}", e),
    }
}
