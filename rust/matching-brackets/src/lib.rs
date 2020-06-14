pub fn brackets_are_balanced(string: &str) -> bool {
    // Remove all chars we do not want
    let chars: String = string
        .chars()
        .filter(|x| ['[', ']', '(', ')', '{', '}'].contains(&x))
        .collect();

    // Split in half..
    let (start, end) = chars.split_at(string.len() / 2);
    // ..reverse end half..
    let end: String = end.chars().rev().collect();
    // ..get iterators from start/end..
    let mut end = end.chars();
    let start = start.chars();
    // ..and compare start end chars with each other!
    let mut all_good = true;
    for x in start {
        let compare = end.next();
        all_good = match x {
            '[' => compare == Some(']'),
            '(' => compare == Some(')'),
            '{' => compare == Some('}'),
            _ => false,
        };
        if !all_good {
            break;
        }
    }
    all_good
}
