// 57a6633153ba33189e000074
pub fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    let mut vec: Vec<(char, i32)> = Vec::new();
    for c in sip.chars() {
        let mut found = false;
        for d in vec.iter_mut() {
            if d.0 == c {d.1 = d.1 + 1;
            found = true;}
        }
        if !found {vec.push((c, 1))};
    }
    return vec;
}

#[cfg(test)]
#[test]
fn test_abradacadabra() {
    assert_eq!(
        ordered_count("abracadabra"),
        vec![('a', 5), ('b', 2), ('r', 2), ('c', 1), ('d', 1)]
    );
}

#[cfg(test)]
#[test]
fn test_banana() {
    assert_eq!(ordered_count("banana"), vec![('b', 1), ('a', 3), ('n', 2)]);
}

#[cfg(test)]
#[test]
fn test_master_solver() {
    assert_eq!(
        ordered_count("i am a master kata solver"),
        vec![
            ('i', 1),
            (' ', 5),
            ('a', 5),
            ('m', 2),
            ('s', 2),
            ('t', 2),
            ('e', 2),
            ('r', 2),
            ('k', 1),
            ('o', 1),
            ('l', 1),
            ('v', 1)
        ]
    );
}

