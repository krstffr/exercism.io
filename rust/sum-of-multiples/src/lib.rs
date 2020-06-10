pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit)
        .filter(|x| factors.iter().filter(|x| **x > 0).any(|y| x % y == 0))
        .sum()
}
