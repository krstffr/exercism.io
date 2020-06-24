pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for i in 0..((digits.len() + 1) - len) {
        result.push(digits[i..(i + (len))].to_string());
    }
    result
}
