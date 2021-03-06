// 54e6533c92449cc251001667

// Implement the function unique_in_order which takes as argument a sequence and returns a list of items without any elements with the same value next to each other and preserving the original order of elements.

// For example:

// uniqueInOrder("AAAABBBCCDAABBB") == {'A', 'B', 'C', 'D', 'A', 'B'}
// uniqueInOrder("ABBCcAD")         == {'A', 'B', 'C', 'c', 'A', 'D'}
// uniqueInOrder([1,2,2,3,3])       == {1,2,3}

pub fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut vec: Vec<T::Item> = Vec::new();
    sequence.into_iter().for_each(|item| {
        if vec.last() != Some(&item) {
         vec.push(item);   
        }
    });
    vec
}

// Top 1 solution is using dedup

// fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
// where
//     T: std::iter::IntoIterator,
//     T::Item: std::cmp::PartialEq + std::fmt::Debug,
// {
//     let mut v: Vec<_> = sequence.into_iter().collect();
//     v.dedup();
//     v
// }


#[cfg(test)]
mod tests {
    
    use super::unique_in_order;

    #[test]
    fn sample_test() {
        assert_eq!(unique_in_order("AAAABBBCCDAABBB".chars()), vec!['A','B','C','D','A','B']);
    }
}
