// create Accoaunt trait to define methods
trait Account<BankAccount> {
    fn deposit(&mut self);
    fn withdraw(&mut self);
    fn balance(&mut self);
}

// BankAccount struct
struct BankAccount<'a> {
    account_number: u32,
    holder_name: &'a str,
    balance: f64,
}

// implement the methods logic
impl BankAccount<'_> {
    // deposit amount function
    // @param amount u32
    // must be positive number
    // returns a Result
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
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
            return Err("not enough amount to withdraw!".to_string());
        }

        // if the user have the amount to withdraw
        // subtract the amount of balance
        self.balance -= amount;
        Ok(())
    }

    // balance function
    // returns a Result with the amount of user's balance
    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // create a bank account instance number one - module 3
    let mut account_number_one = BankAccount {
        account_number: 1,
        holder_name: "The Dude with the First Account",
        balance: 0.0,
    };

    // create a bank account instance number two
    let mut account_number_two = BankAccount {
        account_number: 2,
        holder_name: "The Dude with the Second Account",
        balance: 0.0,
    };

    let deposit_amount = 100.00;
    let withdraw_amount = 50.0;

    // Deposit amount to first account and print the result
    account_number_one.deposit(deposit_amount);
    println!(
        "Deposit of ${} in the account number: {} successfully done!",
        deposit_amount, account_number_one.account_number,
    );

    // Deposit amount to second account and print the result
    account_number_two.deposit(deposit_amount);
    println!(
        "Deposit of ${} in the account number: {} successfully done!",
        deposit_amount, account_number_two.account_number,
    );

    // Withdraw amount to first account and print the result
    account_number_one.withdraw(withdraw_amount).unwrap();
    println!(
        "Withdraw of ${} in the account number: {} successfully done!",
        withdraw_amount, account_number_one.account_number,
    );

    // Deposit amount to second account and print the result
    account_number_two.withdraw(withdraw_amount).unwrap();
    println!(
        "Withdraw of ${} in the account number: {} successfully done!",
        withdraw_amount, account_number_two.account_number,
    );

    // Get the balance of first account and print it
    let balance = account_number_one.balance();
    println!(
        "Balance of account number {} from account name: {} is: ${}",
        account_number_one.account_number, account_number_one.holder_name, balance,
    );

    // Get the balance of second account and print it
    let balance = account_number_two.balance();
    println!(
        "Balance of account number {} from account name: {} is: ${}",
        account_number_two.account_number, account_number_two.holder_name, balance,
    );

    // Must get an error for withdraw without money
    let result = account_number_one.withdraw(200.00);
    if let Ok(()) = result {
        println!(
            "Withdraw of ${} in the account number: {} successfully done!",
            withdraw_amount, account_number_one.account_number,
        );
    } else {
        println!("{:?}", result);
    };
}
