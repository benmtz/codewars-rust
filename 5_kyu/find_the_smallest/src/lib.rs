#[cfg(test)]
mod tests {
    use std::cmp::{min, max};

    fn smallest(n: i64) -> (i64, usize, usize) {
        let n_split = n.to_string();
        let n_split = n_split.chars();
        let mut n_split = n_split.map(|s| s.to_digit( 10).unwrap() as i64).collect();
        let min_index = get_first_lowest(&n_split);
        let switch_to = get_first_diff(&n_split ,n_split.get(min_index).unwrap());

        n_split.swap(min_index, switch_to);
        let n_split: String = n_split.iter().map(|n| std::char::from_digit(*n as u32, 10).unwrap()).collect();
        let n_split: &str = &n_split;
        (i64::from_str_radix(n_split, 10).unwrap(), min(min_index, switch_to), max(switch_to, min_index))
    }

    fn get_first_lowest(nbs: &Vec<i64>) -> usize {
        nbs.iter()
            .enumerate()
            .min_by_key(|&(_, n)| n)
            .unwrap().0
    }

    fn get_first_diff(nbs: &Vec<i64>, diff: &i64) -> usize {
        nbs.iter()
            .enumerate()
            .find(|&(_, n)| n != diff)
            .unwrap_or((0, &0))
            .0
    }

    #[test]
    fn get_first_diff_test() {
        assert_eq!(get_first_diff(&vec![6,5,3,1], &3), 0);
        assert_eq!(get_first_diff(&vec![6,6,6,1], &6), 3);
        assert_eq!(get_first_diff(&vec![2,6,1,2,3,5], &1), 0);
    }

    #[test]
    fn get_first_lowest_test() {
        assert_eq!(get_first_lowest(&vec![2,6,1,2,3,5]), 2);
        assert_eq!(get_first_lowest(&vec![6,5,3,1]), 3);
        assert_eq!(get_first_lowest(&vec![6,5,1,1]), 2);
        assert_eq!(get_first_lowest(&vec![1,1]), 0);
    }

    #[test]
    fn basic_tests() {
        assert_eq!(smallest(209917), (29917, 0, 1));
        assert_eq!(smallest(261235), (126235, 2, 0));
        assert_eq!(smallest(285365), (238565, 3, 1));
    }
}
