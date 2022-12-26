use crate::Solution;
use nom::branch::alt;
use nom::character::complete::{char as n_char, u8 as n_u8};
use nom::combinator::{iterator, map, map_opt};
use nom::IResult;
use std::collections::HashMap;

pub struct Day22;

impl Solution<22> for Day22 {
    type Output = usize;

    fn part1(&self, input: &str) -> Self::Output {
        let (grid, instructions) = input.split_once("\n\n").unwrap();
        let (grid, length, width) = parse_grid(grid);
        let mut instructions = iterator(instructions, parse_instruction);
        let initial = State::get_starting_state(&grid);

        let state = instructions.fold(initial, |state, instruction| match instruction {
            Instruction::Rotate(dir) => state.rotate_by(dir),
            Instruction::Move(steps) => {
                let State {
                    mut row,
                    mut col,
                    direction,
                } = state;
                for _ in 0..steps {
                    let mut next_row = (row as isize + ROW_OFFSETS[direction] + length) % length;
                    let mut next_col = (col as isize + COL_OFFSETS[direction] + width) % width;

                    while grid[next_row as usize][next_col as usize] == Tile::Void {
                        next_row = (next_row + ROW_OFFSETS[direction] + length) % length;
                        next_col = (next_col + COL_OFFSETS[direction] + width) % width;
                    }

                    if grid[next_row as usize][next_col as usize] == Tile::Wall {
                        break;
                    } else {
                        row = next_row as usize;
                        col = next_col as usize;
                    }
                }
                State {
                    row,
                    col,
                    direction,
                }
            }
        });

        state.password()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let (grid, instructions) = input.split_once("\n\n").unwrap();
        let (grid, _, _) = parse_grid(grid);
        let mut instructions = iterator(instructions, parse_instruction);
        let initial = State::get_starting_state(&grid);

        let cube = Cube::from_grid(grid);
        let state = instructions.fold(initial, |state, instruction| match instruction {
            Instruction::Rotate(dir) => state.rotate_by(dir),
            Instruction::Move(steps) => {
                let mut state = state;
                for _ in 0..steps {
                    let next_state = cube.step_state(&state);
                    match cube.grid[next_state.row][next_state.col] {
                        Tile::Empty => {
                            state = next_state;
                        }
                        Tile::Wall => break,
                        Tile::Void => unreachable!("Wandered off into the void somehow"),
                    }
                }
                state
            }
        });

        Some(state.password())
    }
}

struct State {
    row: usize,
    col: usize,
    direction: usize,
}

const LEFT: usize = 3;
const RIGHT: usize = 1;

impl State {
    fn get_starting_state(grid: &Vec<Vec<Tile>>) -> State {
        State {
            row: 0,
            col: grid[0].iter().position(|p| *p == Tile::Empty).unwrap(),
            direction: 0,
        }
    }

    fn password(&self) -> usize {
        (self.row + 1) * 1000 + (self.col + 1) * 4 + self.direction
    }

    fn rotate_by(&self, quarters: usize) -> Self {
        Self {
            row: self.row,
            col: self.col,
            direction: (self.direction + quarters) % 4,
        }
    }
}

struct Cube {
    grid: Vec<Vec<Tile>>,
    side_length: usize,
    face_transitions: [[(usize, usize); 4]; 6],
    face_ids: HashMap<(usize, usize), usize>,
}

impl Cube {
    fn from_grid(grid: Vec<Vec<Tile>>) -> Self {
        let tiles = grid
            .iter()
            .flat_map(|row| row.iter().filter(|tile| tile != &&Tile::Void))
            .count()
            / 6;
        let side_length = (tiles as f64).sqrt() as usize;
        let face_ids = Self::grid_faces(&grid, side_length);

        let mut face_transitions = [[Option::<(usize, usize)>::None; 4]; 6];

        let mut outstanding_transitions = 6 * 4;

        for (&(row, col), id) in face_ids.iter() {
            for direction in 0..4 {
                let next_row = (row as isize + ROW_OFFSETS[direction]) as usize;
                let next_col = (col as isize + COL_OFFSETS[direction]) as usize;

                if let Some(&n_id) = face_ids.get(&(next_row, next_col)) {
                    face_transitions[*id][direction] = Some((n_id, direction));
                    outstanding_transitions -= 1;
                }
            }
        }

        while outstanding_transitions > 0 {
            for id in 0..6 {
                for d in 0..4 {
                    if face_transitions[id][d].is_some() {
                        continue;
                    }

                    let ld = (d + LEFT) % 4;

                    if let Some((n_id, nd)) = face_transitions[id][ld] {
                        let rd = (nd + RIGHT) % 4;

                        if let Some((nn_id, nd)) = face_transitions[n_id][rd] {
                            let ld = (nd + LEFT) % 4;
                            face_transitions[id][d] = Some((nn_id, ld));
                            outstanding_transitions -= 1;
                        }
                    }
                }
            }
        }

        let face_transitions = {
            let mut targets = [[(0, 0); 4]; 6];
            for (from, to) in face_transitions.into_iter().zip(targets.iter_mut()) {
                for (from, to) in from.into_iter().zip(to.iter_mut()) {
                    *to = from.unwrap();
                }
            }
            targets
        };

        Self {
            side_length,
            grid,
            face_transitions,
            face_ids,
        }
    }

    fn grid_faces(grid: &Vec<Vec<Tile>>, side_length: usize) -> HashMap<(usize, usize), usize> {
        let mut face_ids = HashMap::new();

        for (row_id, row) in grid.iter().enumerate() {
            for (col_id, tile) in row.iter().enumerate() {
                if tile != &Tile::Void {
                    let id = Self::face_for(row_id, col_id, side_length);
                    let n = face_ids.len();
                    face_ids.entry(id).or_insert_with(|| n);
                }
            }
        }

        face_ids
    }

    fn face(&self, row: usize, col: usize) -> (usize, usize) {
        Self::face_for(row, col, self.side_length)
    }

    fn face_for(row: usize, col: usize, side_length: usize) -> (usize, usize) {
        (
            (row + side_length) / side_length,
            (col + side_length) / side_length,
        )
    }

    fn step_state(&self, state: &State) -> State {
        let &State {
            row,
            col,
            mut direction,
        } = state;
        let next_row = (row as isize + ROW_OFFSETS[direction]) as usize;
        let next_col = (col as isize + COL_OFFSETS[direction]) as usize;

        let current_face = self.face(row, col);
        let next_face = self.face(next_row, next_col);

        if current_face == next_face {
            State {
                row: next_row,
                col: next_col,
                direction,
            }
        } else {
            let face_id = self.face_ids[&current_face];
            let (next_face_id, next_direction) = self.face_transitions[face_id][direction];

            let mut row = row % self.side_length;
            let mut col = col % self.side_length;

            while direction != next_direction {
                direction = (direction + RIGHT) % 4;
                let next_row = col;
                let next_col = self.side_length - row - 1;
                row = next_row;
                col = next_col;
            }

            let (next_row, next_col) = self
                .face_ids
                .iter()
                .find(|(_, &id)| id == next_face_id)
                .unwrap()
                .0;

            let next_cube_row = (*next_row - 1) * self.side_length;
            let next_cube_col = (*next_col - 1) * self.side_length;

            State {
                row: (next_cube_row as isize
                    + row as isize
                    + (1 - self.side_length as isize) * ROW_OFFSETS[next_direction])
                    as usize,
                col: (next_cube_col as isize
                    + col as isize
                    + (1 - self.side_length as isize) * COL_OFFSETS[next_direction])
                    as usize,
                direction: next_direction,
            }
        }
    }
}

fn parse_grid(input: &str) -> (Vec<Vec<Tile>>, isize, isize) {
    let mut grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    ' ' => Tile::Void,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let length = grid.len() as isize;
    let width = grid.iter().map(Vec::len).max().unwrap() as isize;
    grid.iter_mut()
        .for_each(|row| row.resize_with(width as usize, || Tile::Void));
    (grid, length, width)
}

const ROW_OFFSETS: [isize; 4] = [0, 1, 0, -1];
const COL_OFFSETS: [isize; 4] = [1, 0, -1, 0];

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Void,
}

#[derive(Debug)]
enum Instruction {
    Move(u8),
    Rotate(usize),
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_move, parse_rotate))(input)
}

fn parse_move(input: &str) -> IResult<&str, Instruction> {
    map(n_u8, Instruction::Move)(input)
}

fn parse_rotate(input: &str) -> IResult<&str, Instruction> {
    map_opt(alt((n_char('L'), n_char('R'))), |c| match c {
        'L' => Some(Instruction::Rotate(3)),
        'R' => Some(Instruction::Rotate(1)),
        _ => None,
    })(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
            ...#
            .#..
            #...
            ....
    ...#.......#
    ........#...
    ..#....#....
    ..........#.
            ...#....
            .....#..
            .#......
            ......#.

    10R5L5R10L4R5L5
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Day22.part1(TEST_INPUT), 6032)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day22.part2(TEST_INPUT), Some(5031))
    }
}
