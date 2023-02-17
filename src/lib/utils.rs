pub fn return_truncated(mut string: String, max_length: usize) -> String {
    if string.len() > max_length {
        string.truncate(max_length - 5);
        string.push_str("[...]");
    }
    string
}

