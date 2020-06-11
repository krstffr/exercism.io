pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 3,
        n => {
            let mut next_prime_n = 2;
            let mut x = 4;
            // LOL this is shit!!
            // Should try some non-naive algorithm, read this:
            // https://www.programminglogic.com/testing-if-a-number-is-prime-efficiently/
            let value = loop {
                let mut is_prime = true;
                'inner: for y in 2..x {
                    is_prime = is_prime && x % y != 0;
                    // This speeds up the naive approach a bit!
                    if !is_prime {
                        break 'inner;
                    }
                }
                if is_prime {
                    next_prime_n += 1;
                    if next_prime_n == (n + 1) {
                        break x;
                    }
                }
                x += 1;
            };
            value
        }
    }
}
