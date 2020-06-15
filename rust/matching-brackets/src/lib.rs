// New idea:
// 1. Go backwards, find first "left-wards-pointing" paren
// 2. once found => go right and find corresponding closing tag
// 3. repeat until string is empty
//    if one of the steps fails: is unbalanced!

// TODO: Could use a good chunk of cleanup!!

fn is_left_facing(c: char) -> (bool, char) {
    match c {
        '{' => (true, '}'),
        '[' => (true, ']'),
        '(' => (true, ')'),
        _ => (false, ' '),
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    if string.len() < 2 {
        return true;
    }
    let mut chars: String = string
        .chars()
        .filter(|x| ['[', ']', '(', ')', '{', '}'].contains(&x))
        .collect();

    if chars.len() % 2 != 0 {
        return false;
    }
    let mut pointer = chars.len() - 1;
    let balanced: bool;
    loop {
        let c = chars.chars().nth(pointer).unwrap();
        let (faces_left, find_right) = is_left_facing(c);
        if faces_left {
            let find_in = &chars[pointer..];
            let pos = find_in.find(find_right);
            match pos {
                Some(pos) => {
                    chars.remove(pos + pointer - 1);
                    chars.remove(pointer);
                    if pointer < 1 {
                        balanced = true;
                        break;
                    }
                    pointer -= 1;
                }
                _ => {
                    balanced = false;
                    break;
                }
            }
        } else {
            if pointer < 1 {
                balanced = false;
                break;
            }
            pointer -= 1;
        }
    }
    balanced
}
