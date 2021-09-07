// 54d512e62a5e54c96200019e

// Given a positive number n > 1 find the prime factor decomposition of n. The result will be a string with the following form :

//  "(p1**n1)(p2**n2)...(pk**nk)"

// with the p(i) in ***increasing order*** and ***n(i) empty if n(i) is 1***.

// Example: n = 86240 should return "(2**5)(5)(7**2)(11)"

use std::collections::BTreeMap;

fn prime_factors(n: i64) -> String {
    get_prime_factorization_map(n)
        .iter().map(|(f, pow)| {
        if *pow == 1 {
            format!("({})", f)
        } else {
            format!("({}**{})", f, pow)
        }
    }).collect()
}

fn get_prime_factorization_map(n: i64) -> BTreeMap<i64, i64> {
    let mut b = BTreeMap::new();
    get_prime_factorization(n).iter().for_each(|f| {
       let fact = b.entry(*f).or_insert(0);
        *fact += 1;
    });
    b
}

fn get_prime_factorization(n: i64) -> Vec<i64> {
    let mut vec = Vec::new();
    let mut nm = n;
    let mut candidate = 2;
    while candidate <= nm {
        while nm % candidate == 0 {
            nm /= candidate;
            vec.push(candidate);
        }
        candidate += 1;
    }
    if nm == n {return vec![n]}
    vec
}

// Nothing special in solutions, the first one does factorisation + push format everytime it
// goes to another candidate (more efficient)

#[cfg(test)]
mod tests {
    
    use super::{get_prime_factorization, prime_factors};

    #[test]
    fn get_prime_factors_test() {
        assert_eq!(get_prime_factorization(4), [2, 2]);
        assert_eq!(get_prime_factorization(2), [2]);
        assert_eq!(get_prime_factorization(12), [2, 2, 3]);
        assert_eq!(get_prime_factorization(11), [11]);
        assert_eq!(get_prime_factorization(933_555_431), [7_537, 123_863]);
    }

    #[test]
    fn basics_prime_factors() {
        assert_eq!(prime_factors(7775460), "(2**2)(3**3)(5)(7)(11**2)(17)");
        assert_eq!(prime_factors(17 * 17 * 93 * 677), "(3)(17**2)(31)(677)");
        assert_eq!(prime_factors(933555431), "(7537)(123863)");
    }
}
