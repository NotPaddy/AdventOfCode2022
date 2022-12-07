#![feature(iter_array_chunks)]

use aoc_2022::{Runner, Solution};
use indextree::{Arena, NodeId};

fn main() {
    Runner::new(include_str!("../inputs/day07.txt")).run(&Day7)
}

struct Day7;

impl Solution<7> for Day7 {
    type Output = usize;

    fn part1(&self, input: &str) -> Self::Output {
        let FileSystem {
            fs, directories, ..
        } = parse_fs(input);
        directories
            .iter()
            .map(|d| FsEntry::get_size(&fs, d))
            .filter(|&s| s <= 100_000)
            .sum()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        const TOTAL_DISK_SPACE: usize = 70_000_000;
        const NEEDED_SPACE: usize = 30_000_000;

        let FileSystem {
            fs,
            directories,
            root,
        } = parse_fs(input);

        let total_used = FsEntry::get_size(&fs, &root);
        let to_delete = NEEDED_SPACE - (TOTAL_DISK_SPACE - total_used);

        directories
            .iter()
            .map(|d| FsEntry::get_size(&fs, d))
            .filter(|&s| s >= to_delete)
            .min()
    }
}

struct FileSystem {
    fs: Arena<FsEntry>,
    root: NodeId,
    directories: Vec<NodeId>,
}

fn parse_fs(input: &str) -> FileSystem {
    let mut arena = Arena::new();
    let mut directories: Vec<NodeId> = Vec::new();
    let root = arena.new_node(FsEntry::Dir {
        name: "/".to_owned(),
    });
    let mut cwd = root;

    for line in input.lines() {
        let parts = line
            .trim_start_matches("$ ")
            .split_whitespace()
            .collect::<Vec<_>>();

        match parts[..] {
            ["cd", "/"] => cwd = root,
            ["cd", ".."] => cwd = arena[cwd].parent().unwrap(),
            ["cd", dir] => {
                cwd = cwd
                    .children(&arena)
                    .find(|&c| matches!(arena[c].get(), FsEntry::Dir { name } if name == dir))
                    .unwrap()
            }
            ["dir", name] => {
                let dir = arena.new_node(FsEntry::Dir {
                    name: name.to_owned(),
                });
                cwd.append(dir, &mut arena);
                directories.push(dir);
            }
            ["ls"] => (),
            [size, name] => {
                let size = size.parse::<usize>().unwrap();
                let file = arena.new_node(FsEntry::File {
                    name: name.to_owned(),
                    size,
                });
                cwd.append(file, &mut arena);
            }
            _ => (),
        }
    }

    FileSystem {
        fs: arena,
        root,
        directories,
    }
}

#[derive(Debug)]
enum FsEntry {
    Dir {
        name: String,
    },
    #[allow(dead_code)]
    File {
        name: String,
        size: usize,
    },
}

impl FsEntry {
    fn get_size(fs: &Arena<FsEntry>, entry: &NodeId) -> usize {
        match fs[*entry].get() {
            FsEntry::Dir { .. } => entry.children(fs).map(|c| FsEntry::get_size(fs, &c)).sum(),
            FsEntry::File { size, .. } => *size,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
        "};

    #[test]
    fn test_part1() {
        assert_eq!(Day7.part1(TEST_INPUT), 95437)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day7.part2(TEST_INPUT), Some(24933642))
    }
}
