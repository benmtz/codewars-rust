#[cfg(test)]
mod tests {

    // Your goal in this kata is to implement a difference function, which subtracts one list from another and returns the result.
    //
    // It should remove all values from list a, which are present in list b.
    //
    // array_diff(vec![1,2], vec![1]) == vec![2]
    //
    // If a value is present in b, all of its occurrences must be removed from the other:
    //
    // array_diff(vec![1,2,2,2,3], vec![2]) == vec![1,3]

    use std::collections::HashSet;
    use std::iter::FromIterator;

    fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
        a.into_iter().filter(|x| !b.contains(x)).collect::<Vec<T>>()
    }

    #[test]
    fn returns_expected() {
        assert_eq!(array_diff(vec![1,2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1,2,2], vec![1]), vec![2,2]);
        assert_eq!(array_diff(vec![1,2,2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1,2,2], vec![]), vec![1,2,2]);
        assert_eq!(array_diff(vec![], vec![1,2]), vec![]);
    }
}
