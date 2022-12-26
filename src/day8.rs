use crate::Solution;

pub struct Day8;

impl Solution<8> for Day8 {
    type Output = usize;

    fn part1(&self, input: &str) -> Self::Output {
        let trees = parse_trees(input);

        trees
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(col_idx, _)| check_suitable(row_idx, *col_idx, &trees).0)
                    .count()
            })
            .sum()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let trees = parse_trees(input);

        trees
            .iter()
            .enumerate()
            .filter_map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col_idx, _)| check_suitable(row_idx, col_idx, &trees).1)
                    .max()
            })
            .max()
    }
}

fn parse_trees(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn check_suitable(row_idx: usize, col_idx: usize, trees: &Vec<Vec<u32>>) -> (bool, usize) {
    if row_idx == 0
        || row_idx == trees.len() - 1
        || col_idx == 0
        || col_idx == trees[row_idx].len() - 1
    {
        return (true, 0);
    }

    let mut visible_sides = 4;
    let mut scenic_scores = [0, 0, 0, 0];

    let height = trees[row_idx][col_idx];

    for r in (0..row_idx).rev() {
        scenic_scores[0] += 1;
        if trees[r][col_idx] >= height {
            visible_sides -= 1;
            break;
        }
    }

    for row in trees[row_idx + 1..].iter() {
        scenic_scores[1] += 1;
        if row[col_idx] >= height {
            visible_sides -= 1;
            break;
        }
    }

    for &tree in trees[row_idx][0..col_idx].iter().rev() {
        scenic_scores[2] += 1;
        if tree >= height {
            visible_sides -= 1;
            break;
        }
    }

    for &tree in trees[row_idx][col_idx + 1..].iter() {
        scenic_scores[3] += 1;
        if tree >= height {
            visible_sides -= 1;
            break;
        }
    }

    (visible_sides > 0, scenic_scores.iter().product())
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Day8.part1(TEST_INPUT), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day8.part2(TEST_INPUT), Some(8));
    }
}
