pub mod ownership;

fn main() {
    //mod 1 - ownership
    let string1 = "Rust ";
    let string2 = "rules!";

    println!("{}", ownership::concatenate_strings(string1, string2));
}
