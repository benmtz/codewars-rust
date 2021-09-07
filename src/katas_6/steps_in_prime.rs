// 5613d06cee1e7da6d5000055
pub fn step(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let ug = g as u64;
    if n - m < ug {
       return None
    }
    for start in m..(n - ug) {
        if is_prime(start) && is_prime(start + ug) {
            return Some((start, start + ug));
        }
    }
    None    
}

pub fn is_prime(n: u64) -> bool {
    let last_candidate = 1 + (n as f64).sqrt() as u64;
    for candidate in 2..last_candidate+1 {
        if n % candidate == 0 { return false; }
    }
    true
}

// Some more functional approaches found in solutions

// fn is_prime(p: u64) -> bool {
//   p >= 2 &&
//   (2..)
//   .take_while(|q| q * q <= p)
//   .all(|q| p % q != 0)
// }

// fn step(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
//   (m..n)
//   .map(|p| (p, p + g as u64))
//   .filter(|&(p0, p1)| is_prime(p0) && is_prime(p1))
//   .nth(0)
// }

#[cfg(test)]
mod test {
    
    use super::{is_prime, step};

    #[test]
    fn is_prime_test() {
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(4), false);
    }    

    fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
        assert_eq!(step(g, m, n), exp)
    }

    #[test]
    fn basics_step() {
        testing(2,100,110, Some((101, 103)));
        testing(4,100,110, Some((103, 107)));
        testing(8,30000,100000, Some((30089, 30097)));
        testing(11,30000,100000, None);
        testing(2,10000000,11000000, Some((10000139, 10000141)));
    }    
}
