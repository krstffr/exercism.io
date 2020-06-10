pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        _ => {
            // This will be our string holder.
            let mut song = String::from("");
            // Create the start
            for x in 0..list.len() - 1 {
                let str = String::from("For want of a {x} the {y} was lost.\n")
                    .replace("{x}", list[x])
                    .replace("{y}", list[x + 1]);
                song.push_str(&str);
            }
            // Add the end.
            song.push_str(
                &String::from("And all for the want of a {first}.").replace("{first}", list[0]),
            );
            song
        }
    }
}
