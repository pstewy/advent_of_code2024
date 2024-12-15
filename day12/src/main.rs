use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::process::id;

fn main() {
    let raw_garden = fs::read_to_string("src/input_day_12.txt").unwrap();
    println!("{}", part2(parse(raw_garden)))
}

// Start iterating through, find a start of a region
// and then go search for everything connected to that region
// Return that, mark everything in the region, and skip to the next
// non accounted for region start

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct Point {
    row: i64,
    col: i64,
}

fn parse(raw: String) -> Vec<Vec<String>> {
    raw.split("\n")
        .into_iter()
        .map(|row| row.chars().into_iter().map(|c| c.to_string()).collect())
        .collect()
}

fn part1(garden: Vec<Vec<String>>) -> i64 {
    let mut total = 0;
    let mut regions: HashMap<String, Vec<HashSet<Point>>> = HashMap::new();
    for (row_num, row) in garden.iter().enumerate() {
        for (col_num, col) in row.iter().enumerate() {
            let identifier = col;
            let point = Point {
                row: row_num as i64,
                col: col_num as i64,
            };
            let mut regions_for_identifier = match regions.entry(identifier.clone()) {
                Entry::Occupied(regions_for_identifier) => {
                    if regions_for_identifier
                        .get()
                        .iter()
                        .any(|region| region.contains(&point))
                    {
                        // We have seen this already, skip it
                        continue;
                    }
                    regions_for_identifier.get().clone()
                }
                Entry::Vacant(_) => {
                    vec![]
                }
            };
            let (region_fence, region) = find_all_in_region_part1(&identifier, point, &garden);
            regions_for_identifier.push(region.clone());
            regions.insert(identifier.clone(), regions_for_identifier);
            total += region_fence;
        }
    }
    total
}

fn part2(garden: Vec<Vec<String>>) -> i64 {
    let mut total = 0;
    let mut regions: HashMap<String, Vec<HashSet<Point>>> = HashMap::new();
    for (row_num, row) in garden.iter().enumerate() {
        for (col_num, col) in row.iter().enumerate() {
            let identifier = col;
            let point = Point {
                row: row_num as i64,
                col: col_num as i64,
            };
            let mut regions_for_identifier = match regions.entry(identifier.clone()) {
                Entry::Occupied(regions_for_identifier) => {
                    if regions_for_identifier
                        .get()
                        .iter()
                        .any(|region| region.contains(&point))
                    {
                        // We have seen this already, skip it
                        continue;
                    }
                    regions_for_identifier.get().clone()
                }
                Entry::Vacant(_) => {
                    vec![]
                }
            };
            let (region_fence, region) = find_all_in_region_part2(&identifier, point, &garden);
            regions_for_identifier.push(region.clone());
            regions.insert(identifier.clone(), regions_for_identifier);
            total += region_fence;
        }
    }
    total
}

fn find_all_in_region_part1(
    identifier: &String,
    start_point: Point,
    garden: &Vec<Vec<String>>,
) -> (i64, HashSet<Point>) {
    let mut to_visit: Vec<Point> = vec![start_point];
    let mut visited: HashSet<Point> = HashSet::new();
    let mut perimeter: usize = 0;
    let region_opts = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    while to_visit.len() > 0 {
        let current = to_visit.pop().unwrap();
        if visited.contains(&current) {
            continue;
        }
        // to be in the to_visit list it is in bounds, and  part of the region
        visited.insert(current.clone());
        perimeter += find_sides_of_plot(identifier, &current, garden);

        region_opts
            .iter()
            .map(|(row_opt, col_opt)| Point {
                row: current.row + row_opt,
                col: current.col + col_opt,
            })
            .filter(|p| {
                in_bounds(garden, p) && garden[p.row as usize][p.col as usize].eq(identifier)
            })
            .for_each(|p| to_visit.push(p.clone()));
    }
    ((visited.len() * perimeter) as i64, visited)
}

fn find_all_in_region_part2(
    identifier: &String,
    start_point: Point,
    garden: &Vec<Vec<String>>,
) -> (i64, HashSet<Point>) {
    let mut to_visit: Vec<Point> = vec![start_point];
    let mut visited: HashSet<Point> = HashSet::new();
    let region_opts = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    while to_visit.len() > 0 {
        let current = to_visit.pop().unwrap();
        if visited.contains(&current) {
            continue;
        }
        // to be in the to_visit list it is in bounds, and  part of the region
        visited.insert(current.clone());

        region_opts
            .iter()
            .map(|(row_opt, col_opt)| Point {
                row: current.row + row_opt,
                col: current.col + col_opt,
            })
            .filter(|p| {
                in_bounds(garden, p) && garden[p.row as usize][p.col as usize].eq(identifier)
            })
            .for_each(|p| {
                to_visit.push(p.clone())
            });
    }
    ((visited.len() * find_sides_of_region(identifier.clone(), visited.clone())) as i64, visited)
}

fn find_sides_of_region(identifier: String, region: HashSet<Point>) -> usize {
    // sides == amount of corners. Find all corners
    let mut sides = 0;
    for plot in region.iter() {
       let above_point_in_region = region.contains(&Point{row: plot.row - 1, col: plot.col});
        let left_point_in_region = region.contains(&Point{row: plot.row, col: plot.col - 1});
        let right_point_in_region = region.contains(&Point{row: plot.row, col: plot.col + 1});
        let down_point_in_region = region.contains(&Point{row: plot.row + 1, col: plot.col});
        
        // diagonals 
        let down_right_point_in_region = region.contains(&Point{row: plot.row + 1, col: plot.col + 1});
        let down_left_point_in_region = region.contains(&Point{row: plot.row+1, col: plot.col-1});
        let up_right_point_in_region = region.contains(&Point{row: plot.row-1, col: plot.col+1});
        let up_left_point_in_region = region.contains(&Point{row: plot.row-1, col: plot.col-1});
        
        let prev_size = sides;
        // Is it a top left corner
        if !above_point_in_region && !left_point_in_region {
            sides += 1;
        }
       // Is it a concave diagonal down left corner
        if down_point_in_region && right_point_in_region && !down_right_point_in_region {
            sides += 1;
        }
        
        // Is it a bottom left corner
        if !left_point_in_region && !down_point_in_region {
            sides += 1
        }
        // up right concave?
        if above_point_in_region && right_point_in_region && !up_right_point_in_region {
            sides += 1
        }
        
        // Top Right corner
        if !right_point_in_region && !above_point_in_region {
            sides += 1
        }
        // down left concave? 
        if left_point_in_region && down_point_in_region && !down_left_point_in_region {
            sides += 1
        }
        
        // bottom right corner
        if !down_point_in_region && !right_point_in_region{
            sides += 1
        }
        // up left corner
        if left_point_in_region && above_point_in_region && !up_left_point_in_region {
            sides += 1
        }
        
        println!("({}), Point ({}, {}) added {} sides ", identifier, plot.row, plot.col, sides-prev_size);
    } 
    sides
}

fn find_sides_of_plot(
    identifier: &String,
    start_point: &Point,
    garden: &Vec<Vec<String>>,
) -> usize {
    vec![(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .map(|(row_opt, col_opt)| Point {
            row: start_point.row + row_opt,
            col: start_point.col + col_opt,
        })
        .filter(|p| !in_bounds(garden, p) || !garden[p.row as usize][p.col as usize].eq(identifier))
        .collect::<Vec<Point>>()
        .len()
}

fn in_bounds(garden: &Vec<Vec<String>>, pos: &Point) -> bool {
    pos.row >= 0
        && pos.row <= (garden.len() - 1) as i64
        && pos.col >= 0
        && pos.col <= (garden[0].len() - 1) as i64
}
