use std::fs;

fn main() {
    let file = fs::read_to_string("src/input_day_9.txt").unwrap();
    let parsed = parse(file);
    // println!("{:?}", part1(parsed.clone()));
    println!("{:?}", part2(parsed))
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

fn part2(disk_map: Vec<Marker>) -> usize {
    let mut modified_disk_map: Vec<Marker> = Vec::new();
    let mut idx_end_most_file = disk_map.iter().rposition(|v| !matches!(v, Marker::FreeSpace)).unwrap();
    // have to clone here so we can modify it below
    for (idx, marker) in disk_map.iter().enumerate() {
        if idx > idx_end_most_file {
            // If we get here, there is no more swapping to do. Just loop through the remaining values and calculate the sum
            modified_disk_map.push(marker.clone()); 
        } else {
            match marker {
                Marker::FreeSpace => {
                    // We have free space, search from the back until we have a file block that will fit
                    let count_of_free_space = determine_marker_count(disk_map.clone(), idx, true, Marker::FreeSpace);
                    let mut val = disk_map.get(idx_end_most_file).unwrap().clone().unwrap_a();
                    let mut count_of_end_most_blocks = determine_marker_count(disk_map.clone(), idx_end_most_file, false, Marker::Block(val));
                    let mut move_backwards = idx_end_most_file - count_of_end_most_blocks + 1;
                    while count_of_end_most_blocks > count_of_free_space {
                        // We don't, pull the next end most blocks
                        let next_end_index  = disk_map[..move_backwards].iter().rposition(|v| !matches!(v, Marker::FreeSpace)).unwrap();
                        val = disk_map.get(next_end_index).unwrap().clone().unwrap_a();
                        count_of_end_most_blocks = determine_marker_count(disk_map.clone(), next_end_index, false, Marker::Block(val));
                        move_backwards = move_backwards - count_of_end_most_blocks + 1;
                        if next_end_index <= idx {
                            break
                        }
                    }
                    // did we ever find a file for this spot? 
                    if count_of_end_most_blocks > count_of_free_space {
                        // no, we won't try and put anything here
                        modified_disk_map.push(marker.clone());
                        continue
                    }
                    // Yes, Pull from the back
                    modified_disk_map.push(Marker::Block(val));
                    // Slice the vector here instead so we ignore the most recent Marker we looked at.
                    // My original idea was to just swap the current idx and the right most idx, but then I would have to borrow
                    // disk_map as mutable twice, which is a no-no.
                    idx_end_most_file = disk_map[..idx_end_most_file].iter().rposition(|v| !matches!(v, Marker::FreeSpace)).unwrap();
                }
                Marker::Block(_) => {
                    modified_disk_map.push(marker.clone());
                },
            }
        }
    }
    modified_disk_map.iter().enumerate().fold(0, |acc, (idx, value)| {
        match value {
            Marker::FreeSpace => acc,
            Marker::Block(value) => {
                println!("multiplying {} * {}", idx, value); 
                acc + idx * value
            }
        }
    }) 
}


fn determine_marker_count(disk_map: Vec<Marker>, start_index: usize, forwards: bool, marker_type: Marker) -> usize {
    let mut count = 1;
    let mut start_index = start_index;
   if forwards {
       start_index += 1;
       while start_index < disk_map.len() {
           let next_marker = disk_map.get(start_index).unwrap();
           // I hate this...
           if let Marker::Block(next_val) = next_marker {
               if let Marker::Block(cur_val) = marker_type {
                   if next_val == &cur_val {
                       count += 1;
                       start_index +=1;
                       continue
                   }
               }
           } else if let Marker::FreeSpace = next_marker {
               if let Marker::FreeSpace = marker_type {
                   count += 1;
                   start_index += 1;
                   continue
               }
           }
          return count
       }
   } else {
       start_index -= 1;
       while start_index > 0 {
           let next_marker = disk_map.get(start_index).unwrap();
           // I hate this...
           if let Marker::Block(next_val) = next_marker {
               if let Marker::Block(cur_val) = marker_type {
                   if next_val == &cur_val {
                       count += 1;
                       start_index -=1;
                       continue
                   }
               }
           } else if let Marker::FreeSpace = next_marker {
              if let Marker::FreeSpace = marker_type {
                  count += 1;
                  start_index -=1;
                  continue
              }
           }
           return count
       }
   }
    count
}