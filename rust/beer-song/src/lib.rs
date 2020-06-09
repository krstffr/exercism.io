// POSSIBLY THE WORST CODE OF MY LIFE??
pub fn verse(n: u32) -> String {
    // Bottle numbers as strings...
    let low_u = if n < 1 { 0 } else { n - 1 };
    let n_high = &n.to_string();
    let n_low = &(low_u).to_string();

    // Replace H and L with bottle numbers...
    let result = String::from(
        "H bottles of beer on the wall, H bottles of beer.
Take one down and pass it around, L bottles of beer on the wall.
",
    )
    .replace("H", n_high)
    .replace("L", n_low);

    // Special cases FTW
    let result = if n == 2 {
        String::from(
            "2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.
",
        )
    } else {
        result
    };
    let result = if n == 1 {
        String::from(
            "1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.
",
        )
    } else {
        result
    };
    if n == 0 {
        String::from(
            "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.
",
        )
    } else {
        result
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut verses: String = String::from("");
    for x in (end..start + 1).rev() {
        verses.push_str(&verse(x));
        if x != end {
            verses.push_str("\n");
        }
    }
    verses
}
