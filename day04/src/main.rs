use iter_tools::Itertools;
use std::fs;

type Map = Vec<Vec<char>>;

fn parse_input(input: &str) -> Map {
    let mut map: Map = vec![];
    input.lines().for_each(|line| {
        let v = line.chars().collect_vec();
        map.push(v);
    });
    map
}

// X M A S
fn check_right(x: usize, y: usize, map: &Map) -> bool {
    if (x >= map.len()) || (y >= map[0].len() - 3) {
        return false;
    }
    map[x][y] == 'X' && map[x][y + 1] == 'M' && map[x][y + 2] == 'A' && map[x][y + 3] == 'S'
}

// S A M X
fn check_left(x: usize, y: usize, map: &Map) -> bool {
    if (x >= map.len()) || (y < 3 || y >= map[0].len()) {
        return false;
    }
    map[x][y] == 'X' && map[x][y - 1] == 'M' && map[x][y - 2] == 'A' && map[x][y - 3] == 'S'
}

// S
// A
// M
// X
fn check_up(x: usize, y: usize, map: &Map) -> bool {
    if (x < 3 || x >= map.len()) || (y >= map[0].len()) {
        return false;
    }
    map[x][y] == 'X' && map[x - 1][y] == 'M' && map[x - 2][y] == 'A' && map[x - 3][y] == 'S'
}

// X
// M
// A
// S
fn check_down(x: usize, y: usize, map: &Map) -> bool {
    if (x >= map.len() - 3) || (y >= map[0].len()) {
        return false;
    }
    map[x][y] == 'X' && map[x + 1][y] == 'M' && map[x + 2][y] == 'A' && map[x + 3][y] == 'S'
}

fn check_diagonal_left_up(x: usize, y: usize, map: &Map) -> bool {
    if (x < 3 || x >= map.len()) || (y < 3 || y >= map[0].len()) {
        return false;
    }
    map[x][y] == 'X'
        && map[x - 1][y - 1] == 'M'
        && map[x - 2][y - 2] == 'A'
        && map[x - 3][y - 3] == 'S'
}

fn check_diagonal_left_down(x: usize, y: usize, map: &Map) -> bool {
    if (x >= map.len() - 3) || (y < 3 || y >= map[0].len()) {
        return false;
    }
    map[x][y] == 'X'
        && map[x + 1][y - 1] == 'M'
        && map[x + 2][y - 2] == 'A'
        && map[x + 3][y - 3] == 'S'
}

fn check_diagonal_right_up(x: usize, y: usize, map: &Map) -> bool {
    if (x < 3 || x >= map.len()) || (y >= map[0].len() - 3) {
        return false;
    }
    map[x][y] == 'X'
        && map[x - 1][y + 1] == 'M'
        && map[x - 2][y + 2] == 'A'
        && map[x - 3][y + 3] == 'S'
}

fn check_diagonal_right_down(x: usize, y: usize, map: &Map) -> bool {
    if (x >= map.len() - 3) || (y >= map[0].len() - 3) {
        return false;
    }
    map[x][y] == 'X'
        && map[x + 1][y + 1] == 'M'
        && map[x + 2][y + 2] == 'A'
        && map[x + 3][y + 3] == 'S'
}

fn first_part(map: &Map) -> u32 {
    let mut count = 0u32;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if check_right(x, y, map) {
                count += 1;
            }
            if check_left(x, y, map) {
                count += 1;
            }
            if check_up(x, y, map) {
                count += 1;
            }
            if check_down(x, y, map) {
                count += 1;
            }
            if check_diagonal_left_up(x, y, map) {
                count += 1;
            }
            if check_diagonal_left_down(x, y, map) {
                count += 1;
            }
            if check_diagonal_right_up(x, y, map) {
                count += 1;
            }
            if check_diagonal_right_down(x, y, map) {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let input = fs::read_to_string("input").expect("File not found");
    let input = input.trim();
    assert_eq!(2427, first_part(&parse_input(input)));
    println!("{:?}", first_part(&parse_input(input)));
}
