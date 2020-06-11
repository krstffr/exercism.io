pub fn nth(n: u64) -> u64 {
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

pub fn factors(n: u64) -> Vec<u64> {
    let mut vec = Vec::new();
    let mut prime_n = n / 3;
    let mut check = n;
    loop {
        // IDEA: Start from higher primes and work your way back and then reverse array?
        let prime = nth(prime_n);
        // let next_prime = nth(prime_n + 1);
        println!("Prime: {} Check: {} Mod: {}", prime, check, check % prime);
        if prime < 2 {
            println!("Now prime is 0...");
            vec = Vec::new();
            break;
        }
        if check % prime == 0 {
            println!("Found match actually! {}", prime);
            vec.push(prime);
            check -= prime;
            if check == prime || check == 0 {
                break;
            }
        } else {
            if prime_n == 0 {
                vec = Vec::new();
                break;
            }
            prime_n -= 1;
        }
    }
    vec.reverse();
    vec
}
