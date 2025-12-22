// https://adventofcode.com/2025/day/5

use std::ops::RangeInclusive;

pub fn solve(input: String) -> (String, String) {
    let input = input.trim();
    let mut split = input.split("\n\n");
    let fresh_str = split.next().unwrap();
    let available_str = split.next().unwrap();

    let mut fresh: Vec<_> = fresh_str
        .lines()
        .map(|l| {
            let mut split = l.split('-');
            let first = split.next().unwrap().parse::<u64>().unwrap();
            let second = split.next().unwrap().parse::<u64>().unwrap();
            first..=second
        })
        .collect();

    let available: Vec<_> = available_str
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    let p1 = available
        .iter()
        .filter(|a| fresh.iter().any(|fresh| fresh.contains(a)))
        .count();

    // p2
    // do a multi-stage reduction on the ranges
    let mut merged = true;
    while merged {
        merged = false;

        let mut i = 0;
        while let Some(mut range) = fresh.get(i).cloned() {
            let mut o = i + 1;
            while let Some(other_range) = fresh.get(o) {
                if merge_ranges(&mut range, other_range.clone()) {
                    fresh.remove(o);
                    merged = true;
                } else {
                    o += 1;
                }
            }

            // write back
            fresh[i] = range;
            i += 1;
        }
    }
    let p2: u64 = fresh.iter().map(|r| r.end() - r.start() + 1).sum();

    (p1.to_string(), p2.to_string())
}

/// Merges `other` into `initial` if they overlap.
///
/// Returns `true` iff they overlap.
fn merge_ranges(range: &mut RangeInclusive<u64>, other: RangeInclusive<u64>) -> bool {
    let lower_lt_lower = other.start() <= range.start();
    let lower_gt_lower = other.start() >= range.start();
    let lower_lt_upper = other.start() <= range.end();
    let upper_gt_lower = other.end() >= range.start();
    let upper_lt_upper = other.end() <= range.end();
    let upper_gt_upper = other.end() >= range.end();

    // `other` fully contains `range`
    if lower_lt_lower && upper_gt_upper {
        *range = other;
        return true;
    }

    // `range` fully contains `other`
    if lower_gt_lower && upper_lt_upper {
        return true;
    }

    // partial overlap with `range` being lower
    if lower_gt_lower && lower_lt_upper {
        *range = (*range.start())..=(*other.end());
        return true;
    }

    // partial overlap with `other` being lower
    if lower_lt_lower && upper_gt_lower {
        *range = (*other.start())..=(*range.end());
        return true;
    }

    // otherwise, they are not overlapping
    false
}

#[cfg(test)]
fn expect_merge_range(
    range: RangeInclusive<u64>,
    other: RangeInclusive<u64>,
    expected_range: Option<RangeInclusive<u64>>,
) {
    // test normally
    let mut out_range = range.clone();
    let out = merge_ranges(&mut out_range, other.clone());
    assert_eq!(
        expected_range.is_some(),
        out,
        "merge_range({:?}, {:?}) output was incorrect",
        range,
        other
    );
    assert_eq!(
        expected_range.clone().unwrap_or(range.clone()),
        out_range,
        "merge_range({:?}, {:?}) range was incorrect",
        range,
        other
    );
    // test with other and range reversed
    let mut out_other = other.clone();
    let out = merge_ranges(&mut out_other, range.clone());
    assert_eq!(
        expected_range.is_some(),
        out,
        "merge_range({:?}, {:?}) output was incorrect",
        other,
        range,
    );
    assert_eq!(
        expected_range.clone().unwrap_or(other.clone()),
        out_other,
        "merge_range({:?}, {:?}) range was incorrect",
        other,
        range,
    );
}

#[test]
fn test_merge_ranges() {
    // no overlap
    expect_merge_range(1..=3, 4..=6, None);

    // full overlap
    expect_merge_range(1..=4, 1..=4, Some(1..=4));

    expect_merge_range(2..=3, 1..=4, Some(1..=4));

    expect_merge_range(1..=4, 2..=4, Some(1..=4));
    expect_merge_range(1..=4, 1..=3, Some(1..=4));

    // partial overlap
    expect_merge_range(1..=3, 3..=5, Some(1..=5));
    expect_merge_range(1..=3, 2..=5, Some(1..=5));
}
