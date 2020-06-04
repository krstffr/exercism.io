pub fn raindrops(n: u32) -> String {
    let pling = if n % 3 == 0 { "Pling" } else { "" };
    let plang = if n % 5 == 0 { "Plang" } else { "" };
    let plong = if n % 7 == 0 { "Plong" } else { "" };
    let result = [pling, plang, plong].join("");
    if result == "" {
        n.to_string()
    } else {
        result
    }
}
