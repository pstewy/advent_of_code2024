use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("input_day_6.txt").unwrap();
    let (guard_map, guard) = parse(file);
    let mut part_1_guard = guard.clone();
    println!("{}", part1(&guard_map, &mut part_1_guard));
    println!("{}", part2(&guard_map, guard, part_1_guard.patrolled))
}

#[derive(Default, Clone, PartialEq)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl TryFrom<String> for Direction {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            ">" => Ok(Direction::Right),
            "<" => Ok(Direction::Left),
            "v" => Ok(Direction::Down),
            "^" => Ok(Direction::Up),
            _ => Err("not a direction character".to_string()),
        }
    }
}

#[derive(Clone, Default, Hash, PartialEq, Eq)]
struct Position {
    row: i64,
    col: i64,
}

#[derive(Default, Clone)]
struct Guard {
    direction: Direction,
    position: Position,

    patrolled: HashMap<Position, Vec<Direction>>,
}

impl Guard {
    fn patrol(&mut self) -> bool {
        match self.direction {
            Direction::Up => self.position.row -= 1,
            Direction::Down => self.position.row += 1,
            Direction::Left => self.position.col -= 1,
            Direction::Right => self.position.col += 1,
        }
        let seen = self
            .patrolled
            .get(&self.position)
            .unwrap_or(&vec![])
            .contains(&self.direction);
        if !self.patrolled.contains_key(&self.position) {
            self.patrolled
                .insert(self.position.clone(), vec![self.direction.clone()]);
        } else {
            let mut empty = Vec::new();
            let directions = self.patrolled.get_mut(&self.position).unwrap_or(&mut empty);
            directions.push(self.direction.clone());
        }
        seen
    }

    fn next(&self) -> Position {
        match self.direction {
            Direction::Up => Position {
                row: self.position.row - 1,
                col: self.position.col,
            },
            Direction::Down => Position {
                row: self.position.row + 1,
                col: self.position.col,
            },
            Direction::Left => Position {
                row: self.position.row,
                col: self.position.col - 1,
            },
            Direction::Right => Position {
                row: self.position.row,
                col: self.position.col + 1,
            },
        }
    }

    fn patrolled(&self) -> usize {
        self.patrolled.len()
    }
}

fn parse(raw: String) -> (Vec<Vec<String>>, Guard) {
    let mut output: Vec<Vec<String>> = Vec::new();
    let mut guard = Guard::default();
    for (row, line) in raw.split("\n").enumerate() {
        output.push(Vec::new());
        for (col, c) in line.chars().enumerate() {
            output[row].push(c.to_string());
            if let Ok(direction) = Direction::try_from(c.to_string()) {
                guard = Guard {
                    position: Position {
                        row: row as i64,
                        col: col as i64,
                    },
                    direction,
                    ..Default::default()
                };
            }
        }
    }
    (output, guard)
}

// Take a reference to this guard so we can use it in part2
fn part1(guard_map: &Vec<Vec<String>>, guard: &mut Guard) -> usize {
    loop {
        let unique_positions = guard.patrolled();
        // walk until we walk off the map, or hit an obstacle
        // Can we walk without falling off?
        let next_position = guard.next();
        if next_position.row < 0 || next_position.col < 0 {
            return unique_positions;
        }
        let Some(next_row) = guard_map.get(next_position.row as usize) else {
            return unique_positions;
        };
        let Some(next_char) = next_row.get(next_position.col as usize) else {
            return unique_positions;
        };
        // The next position is in the map. Does it have an obstacle?
        if next_char == "#" {
            // It does. turn, and try again
            guard.direction = guard.direction.turn_right();
            continue;
        }

        // We can patrol!
        guard.patrol();
    }
}

fn part2(
    guard_map: &Vec<Vec<String>>,
    guard: Guard,
    known_path: HashMap<Position, Vec<Direction>>,
) -> usize {
    let mut possible_new_obstacles = 0;
    for (position, _) in known_path {
        let mut cloned_map = guard_map.clone();
        cloned_map[position.row as usize][position.col as usize] = String::from("#");
        if part2_patrol(&cloned_map, guard.clone()) {
            possible_new_obstacles += 1;
        }
    }
    possible_new_obstacles
}

fn part2_patrol(guard_map: &Vec<Vec<String>>, mut guard: Guard) -> bool {
    loop {
        // walk until we walk off the map, or hit an obstacle
        // Can we walk without falling off?
        let next_position = guard.next();
        if next_position.row < 0 || next_position.col < 0 {
            return false;
        }
        let Some(next_row) = guard_map.get(next_position.row as usize) else {
            return false;
        };
        let Some(next_char) = next_row.get(next_position.col as usize) else {
            return false;
        };
        // The next position is in the map. Does it have an obstacle?
        if next_char == "#" {
            // It does. turn, and try again
            guard.direction = guard.direction.turn_right();
            continue;
        }

        // We can patrol!
        let patrolled_already = guard.patrol();
        if patrolled_already {
            return true;
        }
    }
}
