fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
    let perc = percent / 100 as f64;
    let mut year_count: i32 = 0;
    let aug = aug as f64;

    let mut c = p0;
    while c < p {
        c = (c as f64 * ( 1.0 + perc) + aug) as i32;
        year_count += 1;
    }

    return year_count;
}

fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) -> () {
    println!("p0: {:?};", p0);
    println!("percent: {:?};", percent);
    println!("aug: {:?};", aug);
    println!("p: {:?};", p);
    let ans = nb_year(p0, percent, aug, p);
    println!("actual:\n{:?};", ans);
    println!("expect:\n{:?};", exp);
    println!("{};", ans == exp);
    assert_eq!(ans, exp);
    println!("{};", "-");
}

#[cfg(test)]
#[test]
fn basic_tests() {
    dotest(1500, 5.0, 100, 5000, 15);
    dotest(1500000, 2.5, 10000, 2000000, 10);
}
