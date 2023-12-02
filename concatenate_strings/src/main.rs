// method that concanate two strings
// @params:
// string_one -> string slice
// string_two -> string slice
// returns String
pub fn concatenate_strings(string_one: &str, string_two: &str) -> String {
    // creates new string to store the result
    let mut result = String::new();

    // append first and second paramenter to the result string
    result.push_str(string_one);
    result.push_str(string_two);

    // return the result String concatenated
    result
}

fn main() {
    // Module 1 - ownership
    // initialize two string slices
    let string_one = "Rust ";
    let string_two = "rules!";

    // call method to concatenate both and return the result
    println!("{}", concatenate_strings(string_one, string_two));
}
