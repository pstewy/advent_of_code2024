use std::collections::HashSet;
use std::fs;

fn main() {
    let map = parse(fs::read_to_string("src/input_day_10.txt").unwrap());
    println!("{}", part2(&map));
}

// Search through the map, finding all of the zeros and finding all of the 9s
// iterate through each zero, trying to find every 9

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Position {
    x: i64,
    y: i64,
}

struct Map {
    raw: Vec<Vec<i64>>,
    zeros: Vec<Position>,
    nines: Vec<Position>,
}

fn parse(file: String) -> Map {
    let mut raw: Vec<Vec<i64>> = Vec::new();
    let mut zeros: Vec<Position> = Vec::new();
    let mut nines: Vec<Position> = Vec::new();

    file.split("\n").enumerate().for_each(|(row, line)| {
        let mut raw_row: Vec<i64> = Vec::new();
        line.chars().enumerate().for_each(|(col, c)| {
            let val = c.to_digit(10).unwrap_or(100) as i64;
            raw_row.push(val);
            match val {
                0 => zeros.push(Position {
                    x: row as i64,
                    y: col as i64,
                }),
                9 => nines.push(Position {
                    x: row as i64,
                    y: col as i64,
                }),
                _ => (),
            }
        });
        raw.push(raw_row);
    });
    Map { raw, zeros, nines }
}

fn part1(map: &Map) -> i64 {
    let mut total = 0;
    map.zeros.iter().for_each(|p| {
        map.nines.iter().for_each(|n| {
            if can_get_to(map, p, n) {
                total += 1;
            }
        })
    });
    total
}


fn part2(map: &Map) -> i64 {
    let mut total = 0;
    map.zeros.iter().for_each(|p| {
        map.nines.iter().for_each(|n| {
            total += paths(map, p, n)
        })
    });
    total
}

fn can_get_to(map: &Map, start: &Position, end: &Position) -> bool {
    let mut next_nodes: Vec<Position> = Vec::new();
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(start.clone());
    next_nodes.push(start.clone());
    let moves = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    while !next_nodes.is_empty() {
        let current = next_nodes.pop().unwrap();
        if current.eq(&end) {
           return true
        }

        // Search each adjacent node
        moves
            .iter()
            .map(|(row_op, col_op)| Position {
                x: current.x + row_op,
                y: current.y + col_op,
            })
            .filter(|pos| in_bounds(map, pos))
            .into_iter()
            .for_each(|pos| {
                if !visited.contains(&pos) {
                    let next_position_val = map.raw[pos.x as usize][pos.y as usize];
                    let cur_val = map.raw[current.x as usize][current.y as usize];
                    if next_position_val == cur_val + 1 {
                        next_nodes.push(pos.clone());
                        visited.insert(pos);
                    }
                }
            });
    }

    false
}

fn paths(map: &Map, start: &Position, end: &Position) -> i64 {
    let mut total = 0;
    let mut next_nodes: Vec<Vec<Position>> = Vec::new();
    let mut path = vec![start.clone()];
    next_nodes.push(path);
    let moves = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    
    while !next_nodes.is_empty() {
        path = next_nodes.pop().unwrap();
        let last_node = path.last().unwrap();
        if last_node.eq(&end) {
            total += 1;
        }
        // Search each adjacent node
        moves
            .iter()
            .map(|(row_op, col_op)| Position {
                x: last_node.x + row_op,
                y: last_node.y + col_op,
            })
            .filter(|pos| in_bounds(map, pos))
            .into_iter()
            .for_each(|pos| {
                if !visited(&pos, path.clone()) {
                    let next_position_val = map.raw[pos.x as usize][pos.y as usize];
                    let cur_val = map.raw[last_node.x as usize][last_node.y as usize];
                    if next_position_val == cur_val + 1 {
                        let mut new_path = Vec::from(path.clone());
                        new_path.push(pos);
                        next_nodes.push(new_path);
                    }
                   
                } 
            });
    }
    total
}

fn visited(pos: &Position, seen: Vec<Position>) -> bool {
   seen.iter().any(|seen_pos| seen_pos.eq(pos))
}

fn in_bounds(map: &Map, pos: &Position) -> bool {
    pos.x >= 0
        && pos.x <= (map.raw.len() - 1) as i64
        && pos.y >= 0
        && pos.y <= (map.raw[0].len() - 1) as i64
}

