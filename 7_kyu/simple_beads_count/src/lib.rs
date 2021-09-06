#[cfg(test)]
mod tests {

    // https://www.codewars.com/kata/58712dfa5c538b6fc7000569/train/rust

    // Two red beads are placed between every two blue beads. There are N blue beads. After looking at the arrangement below work out the number of red beads.
    // @ @@ @ @@ @ @@ @ @@ @ @@ @
    // Implement count_red_beads(n) (in PHP count_red_beads($n); in Java, Javascript, TypeScript, C, C++ countRedBeads(n)) so that it returns the number of red beads.
    // If there are less than 2 blue beads return 0.

    // My initial solution
    fn count_red_beads(n: u32) -> u32 {
        if n < 2 { 0 } else { (n - 1) * 2}
    }

    // Best solution 
    // fn count_red_beads(n: u32) -> u32 {
    //     match n {
    //     0 | 1 => 0,
    //     _     => ( n - 1) * 2
    //     }
    // }

    #[test]
    fn test() {
      assert_eq!(count_red_beads(0), 0);
      assert_eq!(count_red_beads(1), 0);
      assert_eq!(count_red_beads(3), 4);
      assert_eq!(count_red_beads(5), 8);
    }

}
