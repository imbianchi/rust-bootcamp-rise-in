pub fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::from("");

    result.push_str(s1);
    result.push_str(s2);

    result
}
