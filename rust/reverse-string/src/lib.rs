pub fn reverse(input: &str) -> String {
    input.chars().fold(String::new(), |acc, c| {
        let mut new = String::new();
        new.push(c);
        [new, acc].join("")
    })
}
