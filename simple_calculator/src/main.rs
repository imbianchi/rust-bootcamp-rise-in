use std::io;

// create a Op enum with operators and its types
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
    Unknown,
}

// function to calculate operation using match
// @params: op -> Operation enum
// returns Result
fn calculate(op: Operation) -> Result<f64, String> {
    match op {
        Operation::Add(value1, value2) => Ok(value1 + value2),
        Operation::Subtract(value1, value2) => Ok(value1 - value2),
        Operation::Multiply(value1, value2) => Ok(value1 * value2),
        Operation::Divide(value1, value2) => Ok(value1 / value2),
        Operation::Unknown => Err("Unknow operation.".to_string()),
    }
}

fn main() {
    // module 2 - simple calculator
    // get the first number from user/prompt
    println!("Input the first number: ");
    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line");

    // get the operator from user/prompt
    println!("Input the operator (+, -, /, *) ");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");
    // parse to ref str to compare later with &str
    let operation: &str = &operation.clone();

    // get second number from user/prompt
    println!("Input the second number: ");
    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");

    // parse numbers to f64
    let first_number = first_number.trim().parse().unwrap();
    let second_number = second_number.trim().parse().unwrap();

    // create operation instance
    let op_instance: Operation = match operation {
        "+\n" => Operation::Add(first_number, second_number),
        "*\n" => Operation::Multiply(first_number, second_number),
        "/\n" => Operation::Divide(first_number, second_number),
        "-\n" => Operation::Subtract(first_number, second_number),
        // return an error in case of unknown operator besides -, +, /, *
        _ => Operation::Unknown,
    };

    // call method and get the result
    let result = calculate(op_instance);

    // print the result to the console
    println!("The result is: {}", result.unwrap());
}
