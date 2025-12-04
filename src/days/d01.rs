// https://adventofcode.com/2025/day/1

const DIAL_STARTING_POINT: i32 = 50;
const DIAL_SIZE: i32 = 100;

pub fn solve(input: String) -> (String, String) {
    // parse input
    let instructions: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(line_num, l)| {
            let dir = l.chars().next().expect("line should have at least 1 char");
            let mut amount = (l[1..])
                .parse::<i32>()
                .unwrap_or_else(|_e| panic!("Line {line_num} was not a number"));
            if dir == 'L' {
                amount = -amount;
            }
            amount
        })
        .collect();

    // solve p1
    let mut p1 = 0;
    let mut curr = DIAL_STARTING_POINT;
    for &instr in instructions.iter() {
        curr = (curr + instr).rem_euclid(DIAL_SIZE);
        p1 += (curr == 0) as i32;
    }

    // solve p2
    let mut p2 = 0;
    curr = DIAL_STARTING_POINT;
    for &instr in instructions.iter() {
        p2 += get_zeros(curr, instr);
        curr = (curr + instr).rem_euclid(DIAL_SIZE);
    }

    // return results
    (p1.to_string(), p2.to_string())
}

/// Gets the zero crossings from the current value, applying the given
/// instruction (for p2).
fn get_zeros(curr: i32, instr: i32) -> i32 {
    if instr > 0 {
        // right
        (instr + curr) / DIAL_SIZE
    } else {
        // left
        let delta = curr + instr;
        if delta <= 0 {
            ((-delta) / DIAL_SIZE) + ((curr != 0) as i32)
        } else {
            0
        }
    }
}

/// Tests the outputs to a few `get_zeros()` cases.
#[test]
fn test_get_zeros() {
    // right
    assert_eq!(get_zeros(0, 1), 0);
    assert_eq!(get_zeros(99, 1), 1);
    assert_eq!(get_zeros(0, 99), 0);
    assert_eq!(get_zeros(0, 100), 1);
    assert_eq!(get_zeros(49, 51), 1);
    assert_eq!(get_zeros(49, 52), 1);
    assert_eq!(get_zeros(20, 80), 1);
    assert_eq!(get_zeros(20, 179), 1);
    assert_eq!(get_zeros(20, 180), 2);
    assert_eq!(get_zeros(20, 181), 2);
    assert_eq!(get_zeros(20, 10_000), 100);

    // left
    assert_eq!(get_zeros(20, -19), 0);
    assert_eq!(get_zeros(20, -20), 1);
    assert_eq!(get_zeros(20, -21), 1);
    assert_eq!(get_zeros(20, -119), 1);
    assert_eq!(get_zeros(20, -120), 2);
    assert_eq!(get_zeros(20, -121), 2);
    assert_eq!(get_zeros(90, -89), 0);
    assert_eq!(get_zeros(90, -90), 1);
    assert_eq!(get_zeros(90, -91), 1);
    assert_eq!(get_zeros(0, -2), 0);
    assert_eq!(get_zeros(0, -91), 0);
    assert_eq!(get_zeros(0, -99), 0);
    assert_eq!(get_zeros(0, -100), 1);
    assert_eq!(get_zeros(0, -101), 1);
}
