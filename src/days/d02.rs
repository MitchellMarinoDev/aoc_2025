// https://adventofcode.com/2025/day/2

pub fn solve(input: String) -> (String, String) {
    // parse
    let ranges: Vec<(u64, u64)> = input
        .trim()
        .split(',')
        .map(|range| {
            let mut split = range.split('-');
            let first = split
                .next()
                .expect("range to have a first value")
                .parse()
                .expect("first value to be an int");
            let second = split
                .next()
                .expect("range to have a second value")
                .parse()
                .expect("second value to be an int");
            (first, second)
        })
        .collect();

    // solve p1
    let mut p1 = 0;
    for (lower, upper) in ranges.iter() {
        for val in *lower..=*upper {
            if is_repeated_p1(val) {
                p1 += val;
            }
        }
    }

    // solve p2
    let mut p2 = 0;
    for (lower, upper) in ranges.iter() {
        for val in *lower..=*upper {
            if is_repeated_p2(val) {
                p2 += val;
            }
        }
    }

    (p1.to_string(), p2.to_string())
}

fn is_repeated_p1(val: u64) -> bool {
    let chars: Vec<char> = val.to_string().chars().collect();
    let len = chars.len();

    if len % 2 != 0 {
        // cannot have a repeated substring twice
        return false;
    }

    for i in 0..len / 2 {
        if chars[i] != chars[len / 2 + i] {
            return false;
        }
    }
    true
}

fn is_repeated_p2(val: u64) -> bool {
    let chars: Vec<char> = val.to_string().chars().collect();
    let len = chars.len();

    'substring_iter: for substring_size in 1..=len / 2 {
        if len % substring_size != 0 {
            // must be evenly divisible
            continue;
        }

        let n_copies = len / substring_size;
        if n_copies < 2 {
            // we need at least 2 copies
            continue;
        }

        for c in 0..substring_size {
            let target = chars[c];
            for n in 1..n_copies {
                if chars[c + n * substring_size] != target {
                    continue 'substring_iter;
                }
            }
        }
        return true;
    }

    false
}

#[test]
fn test_is_repeated_p1() {
    assert_eq!(is_repeated_p1(1), false);
    assert_eq!(is_repeated_p1(11), true);
    assert_eq!(is_repeated_p1(111), false);
    assert_eq!(is_repeated_p1(1111), true);

    assert_eq!(is_repeated_p1(1212), true);
    assert_eq!(is_repeated_p1(12121), false);
    assert_eq!(is_repeated_p1(121212), false);
    assert_eq!(is_repeated_p1(1213), false);
    assert_eq!(is_repeated_p1(1221), false);

    assert_eq!(is_repeated_p1(22), true);
    assert_eq!(is_repeated_p1(23), false);
    assert_eq!(is_repeated_p1(2323), true);
    assert_eq!(is_repeated_p1(123123), true);

    // given examples
    assert_eq!(is_repeated_p1(11), true);
    assert_eq!(is_repeated_p1(22), true);
    assert_eq!(is_repeated_p1(99), true);
    assert_eq!(is_repeated_p1(1010), true);
    assert_eq!(is_repeated_p1(1188511885), true);
    assert_eq!(is_repeated_p1(222222), true);
    assert_eq!(is_repeated_p1(446446), true);
    assert_eq!(is_repeated_p1(38593859), true);
}

#[test]
fn test_is_repeated_p2() {
    assert_eq!(is_repeated_p2(1), false);
    assert_eq!(is_repeated_p2(11), true);
    assert_eq!(is_repeated_p2(111), true);
    assert_eq!(is_repeated_p2(1111), true);

    assert_eq!(is_repeated_p2(1212), true);
    assert_eq!(is_repeated_p2(12121), false);
    assert_eq!(is_repeated_p2(121212), true);
    assert_eq!(is_repeated_p2(1213), false);
    assert_eq!(is_repeated_p2(1221), false);

    assert_eq!(is_repeated_p2(22), true);
    assert_eq!(is_repeated_p2(23), false);
    assert_eq!(is_repeated_p2(2323), true);
    assert_eq!(is_repeated_p2(123123), true);

    assert_eq!(is_repeated_p2(11), true);
    assert_eq!(is_repeated_p2(22), true);
    assert_eq!(is_repeated_p2(99), true);
    assert_eq!(is_repeated_p2(111), true);
    assert_eq!(is_repeated_p2(999), true);
    assert_eq!(is_repeated_p2(1010), true);
    assert_eq!(is_repeated_p2(1188511885), true);
    assert_eq!(is_repeated_p2(222222), true);
    assert_eq!(is_repeated_p2(446446), true);
    assert_eq!(is_repeated_p2(38593859), true);
    assert_eq!(is_repeated_p2(565656), true);
    assert_eq!(is_repeated_p2(824824824), true);
    assert_eq!(is_repeated_p2(2121212121), true);
}
