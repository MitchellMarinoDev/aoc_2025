// https://adventofcode.com/2025/day/4

const PAPER: char = '@';
const ADJ_THRESHOLD: usize = 4;
const ADJACENT: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

pub fn solve(input: String) -> (String, String) {
    // parse input
    let mut floor: Vec<Vec<bool>> = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c == PAPER).collect())
        .collect();

    let height = floor.len() as isize;
    let width = floor[0].len() as isize;

    // solve p1
    let mut p1 = 0;
    for y in 0..height {
        for x in 0..width {
            if !floor[y as usize][x as usize] {
                // no paper here. skip
                continue;
            }

            if n_adj_paper(&floor, height, width, y, x) < ADJ_THRESHOLD {
                p1 += 1;
            }
        }
    }

    // solve p2
    let mut old_p2 = 1;
    let mut p2 = 0;
    while old_p2 != p2 {
        old_p2 = p2;

        for y in 0..height {
            for x in 0..width {
                if !floor[y as usize][x as usize] {
                    // no paper here. skip
                    continue;
                }

                if n_adj_paper(&floor, height, width, y, x) < ADJ_THRESHOLD {
                    p2 += 1;
                    floor[y as usize][x as usize] = false;
                }
            }
        }
    }

    (p1.to_string(), p2.to_string())
}

fn n_adj_paper(floor: &Vec<Vec<bool>>, height: isize, width: isize, y: isize, x: isize) -> usize {
    ADJACENT
        .iter()
        .filter(|(dir_x, dir_y)| {
            let adj_x = x + *dir_x;
            let adj_y = y + *dir_y;
            (0..width).contains(&adj_x)
                && (0..height).contains(&adj_y)
                && floor[adj_y as usize][adj_x as usize]
        })
        .count()
}
