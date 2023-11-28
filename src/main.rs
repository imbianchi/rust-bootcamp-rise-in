// method that concanate two string
// @params:
// s1 -> string slice
// s2 -> string slice
// returns String
pub fn concatenate_strings(s1: &str, s2: &str) -> String {
    // creates new string to store the result
    let mut result = String::new();

    // append first and second paramenter to the result string
    result.push_str(s1);
    result.push_str(s2);

    // return the result String concatenated
    result
}

fn main() {
    // mod 1 - ownership
    // initialize two string slices
    let string1 = "Rust ";
    let string2 = "rules!";

    // call method to concatenate both and return the result
    println!("{}", concatenate_strings(string1, string2));
}
