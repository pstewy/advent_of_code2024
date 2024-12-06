use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("input_day_6.txt").unwrap();
    let (guard_map, guard) = parse(file);
    println!("{}", part1(&guard_map, guard))
}

#[derive(Default)]
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

#[derive(Clone, Default)]
struct Position {
    row: i64,
    col: i64
}


#[derive(Default)]
struct Guard {
    direction: Direction,
    position: Position,

    patrolled: HashMap<String, ()>
}

impl Guard {
    fn patrol(&mut self) {
       match self.direction {
           Direction::Up => self.position.row -= 1,
           Direction::Down => self.position.row += 1,
           Direction::Left => self.position.col -= 1,
           Direction::Right => self.position.col += 1
       }
        self.patrolled.insert(format!("({},{})", self.position.row, self.position.col), ());
    }

    fn next(&self) -> Position {
        match self.direction {
            Direction::Up => Position {row: self.position.row - 1, col: self.position.col},
            Direction::Down => Position {row: self.position.row + 1, col: self.position.col},
            Direction::Left => Position {row: self.position.row, col: self.position.col - 1},
            Direction::Right => Position {row: self.position.row, col: self.position.col + 1},
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
                guard = Guard{position: Position{row: row as i64, col: col as i64}, direction, ..Default::default()};
            }
        }
    }
    (output, guard)
}

fn part1(guard_map: &Vec<Vec<String>>, mut guard: Guard) -> usize {
    loop {
        let unique_positions = guard.patrolled();
        // walk until we walk off the map, or hit an obstacle
        // Can we walk without falling off?
        let next_position = guard.next();
        if next_position.row < 0 || next_position.col < 0 {
            return unique_positions
        }
        let Some(next_row) = guard_map.get(next_position.row as usize) else {
            return unique_positions
        };
        let Some(next_char) = next_row.get(next_position.col as usize) else {
            return unique_positions
        };
        // The next position is in the map. Does it have an obstacle?
        if next_char == "#" {
            // It does. turn, and try again
            guard.direction = guard.direction.turn_right();
            continue
        }

        // We can patrol!
        guard.patrol();
    }
}

