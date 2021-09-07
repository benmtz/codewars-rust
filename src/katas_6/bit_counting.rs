// 526571aae218b8ee490006f4
// Okay...
// n.count_ones() was enough
// n.matches("1") was possible too

fn count_bits(n: i64) -> u32 {
    format!("{:b}", n).chars().filter(|x| x.to_string() == "1").count() as u32
}

#[cfg(test)]
#[test]
fn returns_expected() {
    assert_eq!(count_bits(0), 0);
    assert_eq!(count_bits(4), 1);
    assert_eq!(count_bits(7), 3);
    assert_eq!(count_bits(9), 2);
    assert_eq!(count_bits(10), 2);
}
