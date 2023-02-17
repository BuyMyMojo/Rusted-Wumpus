pub fn return_truncated(string: String, max_length: usize) -> String {
    if string.len() > max_length-5 {
        format!("{}[...]", &string[0..max_length])
    } else {
        string
    }
}