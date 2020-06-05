
// Wow this is getting bad!
fn is_prime(n: u32) -> bool {
    let mut found = false;
    for x in 3..n {
        if x == n { break } else { () }
        found = 
    }
    found
}

pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 3,
        n => {
            let mut res = 0;
            for x in 2..n {
                res = x
            }
            res
        }
    }
}
