use std::collections::HashMap;
use std::fs;

fn main() {
    let f = fs::read_to_string("src/input_day_8.txt").unwrap();
    let (frequencies, bottom_right) = parse(f);
    println!("{:?}", part1(frequencies, &bottom_right));
}

// Parse and return the frequency points, and the map boundary
fn parse(file: String) -> (HashMap<String, Vec<Point>>, Point)  {
    let lines = file.split("\n").collect::<Vec<&str>>();
    let mut output: HashMap<String, Vec<Point>> = HashMap::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, character) in line.chars().enumerate() {
            if !character.to_string().eq(&".".to_string()) {
                let frequency = Point {row: row as i64, col: col as i64};
                if !output.contains_key(&character.to_string()) {
                    output.insert(character.to_string(), vec!(frequency));
                } else {
                    let mut frequencies = output.get_mut(&character.to_string()).unwrap();
                    frequencies.push(frequency);
                }
            }
        }
    }
    (output, Point{row: (lines.len() - 1) as i64, col: (lines[0].len() - 1) as i64 })
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    row: i64,
    col: i64,
}

fn part1(frequencies_map: HashMap<String, Vec<Point>>, bottom_right: &Point) -> usize {
    let mut found_nodes: HashMap<Point, ()> = HashMap::new();
   // We know the location of each frequency, now search each of them and see if there are antinodes on the map
    for (identifier, frequencies) in frequencies_map.iter() {
       if frequencies.len() == 1 {
           // There is only one, there cannot be any nodes
           continue
       }
       // Not the most efficient, but: for each coord, loop through the other coords and determine else if
        // they create nodes
        for (idx, cur_frequency) in frequencies.iter().enumerate() {
            for next_frequency in frequencies.iter().skip(idx) {
                if next_frequency.eq(cur_frequency) {
                   continue
                }
                println!("Identifier: {:?}, Prev frequency is {:?}. Current frequency is {:?}", identifier, cur_frequency, &next_frequency);
                for nodes in determine_antinode(cur_frequency, &next_frequency, bottom_right) {
                    found_nodes.insert(nodes, ());
                }
            }
        }
    }
    print_map_with_nodes(bottom_right.row, bottom_right.col, &frequencies_map, &found_nodes);
    found_nodes.len() + frequencies_map.iter().map(|(_, v)| {
        let mut count = 0;
        for p in v {
            if !found_nodes.contains_key(p) {
                count += 1
            }
        }
        count
    }).sum::<usize>()
}

fn print_map_with_nodes(row_limit: i64, col_limit: i64, frequencies_map: &HashMap<String, Vec<Point>>, found_nodes: &HashMap<Point, ()>) {
    let mut line = String::new();
    for row in 0..=row_limit {
        for col in 0..=col_limit {
            let mut added = false;
            if found_nodes.contains_key(&Point { row: row as i64, col: col as i64 }) {
                line += "#";
                added = true;
                continue
            }
            for (identifier, frequencies) in frequencies_map.iter() {
                if frequencies.contains(&Point { row: row as i64, col: col as i64 }) {
                    line += format!("{}", identifier).as_ref();
                    added = true;
                    break
                }
            }
            if !added {
                line += ".";
            }
        }
        println!("{}", line);
        line = "".to_string();
    }
}

fn determine_antinode(first: &Point, second: &Point, bottom_right: &Point) -> Vec<Point> {
    if first.col > second.col {
        compute_right_down_to_left(first, second, bottom_right)
    } else {
        compute_left_down_to_right(first, second, bottom_right)
    }
}

fn compute_left_down_to_right(first: &Point, second: &Point, bottom_right: &Point) -> Vec<Point> {
    let mut nodes: Vec<Point> = Vec::new();
    let mut first = first.clone();
    let mut second = second.clone();
    let row_difference = (second.row - first.row).abs();
    let col_difference = (second.col - first.col).abs();
    // for part 2, keep checking for points until we fall off the map
    loop {
        let first_difference = Point{row: first.row - row_difference, col: first.col - col_difference};
        if in_bounds(&first_difference, bottom_right) {
            println!("found one at ({},{})", first_difference.row, first_difference.col);
            nodes.push(first_difference.clone());
        } else {
            break
        }
        first = first_difference
    }
    loop {
        let second_difference = Point{row: second.row + row_difference, col: second.col + col_difference};
        if in_bounds(&second_difference, bottom_right) {
            println!("found one at ({},{})", second_difference.row, second_difference.col);
            nodes.push(second_difference.clone());
        } else {
            break
        }
        second = second_difference
    }
    nodes
}

fn compute_right_down_to_left(first: &Point, second: &Point, bottom_right: &Point) -> Vec<Point>{
    let mut nodes: Vec<Point> = Vec::new();
    let mut first = first.clone();
    let mut second = second.clone();
    let row_difference = (second.row - first.row).abs();
    let col_difference = (second.col - first.col).abs();
    loop {
        let first_difference = Point{row: first.row - row_difference, col: first.col + col_difference};
        if in_bounds(&first_difference, bottom_right) {
            println!("found one at ({},{})", first_difference.row, first_difference.col);
            nodes.push(first_difference.clone());
        } else {
            break
        }
        first = first_difference
    }

    loop {
        let second_difference = Point{row: second.row + row_difference, col: second.col - col_difference};
        if in_bounds(&second_difference, bottom_right) {
            println!("found one at ({},{})", second_difference.row, second_difference.col);
            nodes.push(second_difference.clone());
        } else {
            break
        }
        second = second_difference
    }

    nodes
}

fn in_bounds(p: &Point, bottom_right: &Point) -> bool {
   p.row >= 0 && p.col >= 0 && p.row <= bottom_right.row && p.col <= bottom_right.col
}