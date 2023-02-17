pub fn return_truncated(string: String, max_length: usize) -> String {
    if string.len() > max_length {
        string.chars().take(max_length - 5).chain("[...]".chars()).collect()
    } else {
    string
    }
}
