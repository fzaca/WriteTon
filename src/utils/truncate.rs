pub fn truncate_string(string: &str, max_length: usize, append_ellipsis: bool) -> String {
    if string.len() <= max_length {
        string.to_string()
    } else {
        let mut truncated_string = string[..max_length].to_string();
        if append_ellipsis {
            truncated_string.push_str("...");
        }
        truncated_string
    }
}
