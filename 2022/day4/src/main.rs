use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn read_lines(filename: &std::path::Path) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    Ok(lines)
}

#[derive(Debug)]
struct Task {
    min: i32,
    max: i32,
}

impl Task {
    fn from_str(input: &str) -> Task {
        let split: Vec<&str> = input.split('-').collect();
        if split.len() != 2{
            panic!("unable to find range delimited");
        }

        let min: i32 = split.get(0).unwrap().parse().expect("unable to parse min of task");
        let max: i32 = split.get(1).unwrap().parse().expect("unable to parse max of task");

        return Task {  min,  max };
    }

    fn fully_contains(&self, other: &Task) -> bool {
        return other.min >= self.min && other.max <= self.max;
    }

    fn in_range(&self, n: i32) -> bool{
        n >= self.min && n <= self.max
    }

    fn overlaps(&self, other: &Task) -> bool{
        return self.in_range(other.min) 
        || self.in_range(other.max) ;
    }
}

fn main() {
    let p = std::path::Path::new("input.txt");
    let lines: Vec<String> = read_lines(p)
        .expect("Failed to read input file")
        .enumerate()
        .flat_map(|(index, x)| {
            if x.is_ok() {
                [x.unwrap()]
            } else {
                panic!("Error reading line {index}")
            }
        })
        .collect();

    let parsed_lines: Vec<Vec<Task>>  = lines.iter()
        .map(|s| s.split(",").map(Task::from_str).collect())
        .collect();

    let number_of_fully_contained_pairs: i32 = parsed_lines.iter().map(|p| {
        let left = p.get(0).unwrap();
        let right = p.get(1).unwrap();
        let fully_contains = left.fully_contains(right) || right.fully_contains(left);
        if fully_contains
        {
            // println!("left={:?} right={:?} with fully contains={fully_contains}", left, right);
            return 1;
        }
        return 0;
    }).sum();

    println!("part1: The number of fully contained pairs={number_of_fully_contained_pairs}"); // 444

    let number_of_fully_contained_pairs: i32 = parsed_lines.iter().map(|p| {
        let left = p.get(0).unwrap();
        let right = p.get(1).unwrap();
        let overlaps = left.overlaps(right) || right.overlaps(left);
        if overlaps
        {
            return 1;
        }
        // println!("left={:?} right={:?} with overlaps={overlaps}", left, right);
        return 0;
    }).sum();

    println!("part2: The number of overlapping pairs={number_of_fully_contained_pairs}"); // 801
}
