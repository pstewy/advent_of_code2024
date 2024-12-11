use std::fs;

fn main() {
    let file = fs::read_to_string("src/input_day_9.txt").unwrap();
    println!("{:?}", part1(parse(file)))
}

#[derive(Clone)]
enum Marker {
    FreeSpace,
    Block(usize)
}

impl Marker {
    fn unwrap_a(self) -> usize {
        match self {
            Marker::FreeSpace => panic!("shouldn't have unwrapped"),
            Marker::Block(v) => v.to_owned(),
        }
    }
}

fn parse(file: String) -> Vec<Marker> {
    let mut output: Vec<Marker> = vec![];
    let mut is_file = true;
    let mut file_id = 0;
    for c in file.chars() {
        let number_of_spaces = c.to_string().parse::<usize>().unwrap();
        let marker = match is_file {
           true => {
               let block = Marker::Block(file_id);
               file_id += 1;
               block
           },
            false => Marker::FreeSpace,
        };
        for i in 0..number_of_spaces {
            output.push(marker.clone());
        }
        is_file = !is_file;
    }
    output
}

fn part1(disk_map: Vec<Marker>) -> usize {
    let mut total = 0;
    let mut idx_end_most_file = disk_map.iter().rposition(|v| !matches!(v, Marker::FreeSpace)).unwrap();
    // have to clone here so we can modify it below
    for (idx, marker) in disk_map.iter().enumerate() {
        if idx > idx_end_most_file {
           break
        }
        match marker {
            Marker::FreeSpace => {
                // We have free space. Pull from the back
                let val = disk_map.get(idx_end_most_file).unwrap().clone().unwrap_a();
                println!("multiplying {} * {}", idx, val);
                total += idx * val;
                // Slice the vector here instead so we ignore the most recent Marker we looked at. 
                // My original idea was to just swap the current idx and the right most idx, but then I would have to borrow 
                // disk_map as mutable twice, which is a no-no.
                idx_end_most_file = disk_map[..idx_end_most_file].iter().rposition(|v| !matches!(v, Marker::FreeSpace)).unwrap();
            }
            Marker::Block(value) => {
                println!("multiplying {} * {}", idx, value.to_owned());
                total += idx * value.to_owned()
            },
        }
    }
    total
}