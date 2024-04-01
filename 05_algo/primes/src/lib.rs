pub fn primes(limit: usize) -> (usize, usize) {
    let mut count = 0;
    let mut max = 0;

    let mut primes_map = vec![true; limit];

    let sqrt_limit = f64::sqrt(limit as f64).floor() as usize;

    for i in 2..=sqrt_limit {
        if primes_map[i] {
            let mut j = i * i;

            while j < limit {
                primes_map[j] = false;
                j += i;
            }
        }
    }

    for (i, value) in primes_map.iter().enumerate().skip(2) {
        if *value {
            count += 1;
            max = i;
        }
    }

    (count, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_works() {
        let (pc, max) = primes(1024);
        assert_eq!(pc, 172);
        assert_eq!(max, 1021);
    }
}
