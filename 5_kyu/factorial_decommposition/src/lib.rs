#[cfg(test)]
mod tests {

    // The aim of the kata is to decompose n! (factorial n) into its prime factors.
    //
    // Examples:
    //
    // n = 12; decomp(12) -> "2^10 * 3^5 * 5^2 * 7 * 11"
    // since 12! is divisible by 2 ten times, by 3 five times, by 5 two times and by 7 and 11 only once.
    //
    // n = 22; decomp(22) -> "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19"
    //
    // n = 25; decomp(25) -> 2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23
    //
    // Prime numbers should be in increasing order. When the exponent of a prime is 1 don't put the exponent.

    use std::collections::BTreeMap;

    fn decomp(n: i32) -> String {

        if n == 1 { return String::from("1"); }

        let mut b_tree_map: BTreeMap<i32, i32> = BTreeMap::new();

        for nb in 2..(n+1) {
            // getting factorization
            let mut nmb = nb;
            let mut candidate = 2;
            while candidate <= nmb {
                while nmb % candidate == 0 {
                    nmb /= candidate;
                    let tree_candidate = b_tree_map.entry(candidate).or_insert(0);
                    *tree_candidate += 1;
                }
                candidate += 1;
            }
        }

        b_tree_map
            .iter()
            .map(|(key, exp)| {
                if *exp == 1 {
                    format!("{}", key)
                } else {
                    format!("{}^{}", key, exp)
                }
            }).collect::<Vec<_>>().join(" * ")
    }

    #[test]
    fn basic_tests() {
        assert_eq!(decomp(17), "2^15 * 3^6 * 5^3 * 7^2 * 11 * 13 * 17");
        assert_eq!(decomp(5),"2^3 * 3 * 5");
        assert_eq!(decomp(22),"2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19");
        assert_eq!(decomp(14),"2^11 * 3^5 * 5^2 * 7^2 * 11 * 13");
        assert_eq!(decomp(25),"2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23");

    }
}
