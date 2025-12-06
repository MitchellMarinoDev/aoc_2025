// https://adventofcode.com/2025/day/3

const BASE_10: u32 = 10;

pub fn solve(input: String) -> (String, String) {
    // parse input
    let banks: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(BASE_10).expect("char to be a digit"))
                .collect()
        })
        .collect();

    // solve p1
    let p1: u64 = banks.iter().map(|bank| get_joltage2(bank)).sum();

    // solve p2
    let p2: u64 = banks.iter().map(|bank| get_joltage_n(12, bank)).sum();

    (p1.to_string(), p2.to_string())
}

fn get_joltage2(bank: &[u32]) -> u64 {
    let (i1, v1) = bank[..bank.len() - 1]
        .iter()
        .copied()
        .enumerate()
        // we want the first max result, so we need to reverse the iter
        .rev()
        .max_by_key(|(_idx, v)| *v)
        .expect("bank not to be empty");
    let v2 = bank[i1 + 1..]
        .iter()
        .copied()
        .max()
        .expect("bank not to be empty");

    v1 as u64 * 10 + v2 as u64
}

fn get_joltage_n(n: usize, bank: &[u32]) -> u64 {
    if n == 0 {
        return 0;
    }

    let (i, v) = bank[..=bank.len() - n]
        .iter()
        .copied()
        .enumerate()
        // we want the first max result, so we need to reverse the iter
        .rev()
        .max_by_key(|(_idx, v)| *v)
        .expect("bank not to be empty");

    v as u64 * 10u64.pow(n as u32 - 1) + get_joltage_n(n - 1, &bank[i + 1..])
}

#[cfg(test)]
fn into_bank(val: u64) -> Vec<u32> {
    // not the most effiecient way to do it,
    // but this is just a helper for tests
    val.to_string()
        .chars()
        .map(|c| c.to_digit(BASE_10).expect("char to be a digit"))
        .collect()
}

#[test]
fn test_get_joltage2() {
    assert_eq!(get_joltage2(&into_bank(987654321111111)), 98);
    assert_eq!(get_joltage2(&into_bank(811111111111119)), 89);
    assert_eq!(get_joltage2(&into_bank(234234234234278)), 78);
    assert_eq!(get_joltage2(&into_bank(818181911112111)), 92);
}

#[test]
fn test_get_joltage12() {
    assert_eq!(get_joltage_n(12, &into_bank(987654321111111)), 987654321111);
    assert_eq!(get_joltage_n(12, &into_bank(811111111111119)), 811111111119);
    assert_eq!(get_joltage_n(12, &into_bank(234234234234278)), 434234234278);
    assert_eq!(get_joltage_n(12, &into_bank(818181911112111)), 888911112111);
}
