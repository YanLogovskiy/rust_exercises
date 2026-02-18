pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    if n == 1 {
        return 3;
    }
    let mut primes: Vec<u32> = vec![2, 3];
    for i in (5..).step_by(2) {
        let mut is_prime = true;
        for p in primes.iter() {
            if p.pow(2) > i {
                is_prime = true;
                break;
            }
            if i % *p == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
            if primes.len() == (n + 1) as usize {
                return i;
            }
        }
    }
    unreachable!()
}
