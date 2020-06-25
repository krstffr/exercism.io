pub fn reply(message: &str) -> &str {
    let mut found = match message.chars().last() {
        Some('?') => Some("Sure."),
        Some(_) => None,
        None => None,
    };
    let without_white_space = message.replace('\t', "").replace(' ', "").replace('\r', "");
    println!("{}/{}", without_white_space, without_white_space.len());
    if without_white_space.len() < 2 && found == None {
        found = Some("Fine. Be that way!");
    }
    let all_caps = message.to_uppercase() == message;
    if all_caps && found == None {
        found = Some("Whoa, chill out!");
    }
    if all_caps && found == Some("Sure.") {
        found = Some("Calm down, I know what I'm doing!");
    }
    match found {
        Some(x) => x,
        None => "Whatever.",
    }
}
