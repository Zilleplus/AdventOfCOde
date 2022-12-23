use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

use backtrace::Backtrace;

fn read_lines(filename: &std::path::Path) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    Ok(lines)
}

#[derive(Debug, Clone)]
struct Stacks {
    stack: Vec<Vec<char>>,
}

#[derive(Debug, Clone)]
struct Move {
    count: u32,
    from: u32,
    to: u32,
}

impl Move {
    fn new(count: u32, from: u32, to: u32) -> Move {
        assert!(from != to); // making sure this is not null move
        return Move { count, from, to };
    }
}

impl Stacks {
    fn parse_indices(line: &str) -> u32 {
        let indices = line.split(' ').filter(|s| s != &"").last();

        if let Some(index) = indices {
            return index.parse().unwrap();
        }

        panic!("unable to find any indices in the file");
    }

    fn new(size: u32) -> Stacks {
        let casted_size = usize::try_from(size).unwrap();
        println!("Creating {casted_size} stacks.");
        Stacks {
            stack: std::iter::repeat(Vec::new()).take(casted_size).collect(),
        }
    }

    fn parse_stack_line(&mut self, line: &str) {
        let mut cursor: usize = 0;
        for s in self.stack.iter_mut() {
            let c = line.chars().nth(cursor).unwrap();
            if c == '[' {
                s.push(line.chars().nth(cursor + 1).unwrap())
            } else {
                // this is empty spot, we should do nothing and skip it.
            }
            cursor = cursor + 4; // on space and 4
        }
    }

    fn deserialize(lines: Vec<&String>) -> Stacks {
        let l = &lines.len();
        let indices_lines = lines.get(l - 1).unwrap();
        let stack_size = Stacks::parse_indices(indices_lines);

        let mut stacks = Stacks::new(stack_size);

        for l in lines.iter().take(l - 1).rev() {
            stacks.parse_stack_line(l);
        }

        return stacks;
    }

    fn pop(&mut self, count: u32, from: u32) -> Vec<char> {
        let source_stack = self
            .stack
            .get_mut(usize::try_from(from).unwrap())
            .expect("invalid source on move");

        let vals: Vec<char> = (0..count).flat_map(|_i| source_stack.pop()).collect();
        if vals.len() != usize::try_from(count).expect("can't convert count") {
            panic!("Not enough stuff on stack {from} -> {count} needed but only {} avialable. \n backtrace={:?}", vals.len(), Backtrace::new());
        }

        return vals;
    }

    fn apply_move(&mut self, m: Move, reverse: bool) -> bool {
        let vals = self.pop(m.count, m.from);

        let maybe_target_stack = self.stack.get_mut(usize::try_from(m.to).unwrap());
        if let Some(target_stack) = maybe_target_stack {
            // We have to convert to vector, as a normal and reverse iterator have different type.
            // Same problem as with C++ ranges.
            let vals = if !reverse {vals} else {vals.into_iter().rev().collect()};
            for val in vals {
                target_stack.push(val);
            }
            return true;
        }

        return false;
    }

    fn print_stacks(&self) {
        let tops = self.stack.iter().map(|s| s.iter().last());
        let msg = tops.fold("".to_owned(), |mut buf, new| {
            buf.push(*new.unwrap_or(&'?'));
            return buf;
        });
        println!("{msg}");
    }
}

fn parse_move(line: &String) -> Move {
    // example: "move 1 from 9 to 4"
    let tokes: Vec<&str> = line.split(' ').collect();

    // if the syntax is correct we find
    let count: u32 = tokes.get(1).unwrap().parse().unwrap();

    // from -> index 3
    let from: u32 = tokes.get(3).unwrap().parse().unwrap();

    // to -> index 5
    let to: u32 = tokes.get(5).unwrap().parse().unwrap();

    // Substract the id's by 1, as the index of the first element is
    // zero in rust.
    Move::new(count, from - 1, to - 1)
}

fn main() {
    let p = Path::new("input.txt");
    let lines: Vec<String> = read_lines(p)
        .expect("unable to read lines from input.txt")
        .enumerate()
        .map(|(line_index, x)| {
            x.unwrap_or_else(|e| panic!("unable to read line {line_index} with error={e}"))
        })
        .collect();

    let serialized_stacks: Vec<&String> = lines
        .iter()
        .take_while(|l| l != &&("".to_owned()))
        .collect();
    let file_stacks = Stacks::deserialize(serialized_stacks);

    let moves: Vec<Move> = lines
        .iter()
        .skip_while(|l| l != &&(""))
        .skip(1)
        .map(|l| parse_move(l))
        .collect();

    let mut part1_stacks = file_stacks.clone();
    for m in moves.clone() {
        // println!("Applying move: {:?}", part1_stacks);
        part1_stacks.apply_move(m.clone(), false);
        // println!("{:?}", m);
    }
    part1_stacks.print_stacks(); // WHTLRMZRC

    let mut part2_stacks = file_stacks.clone();
    for m in moves {
        //println!("Applying move: {:?}", part2_stacks);
        part2_stacks.apply_move(m.clone(), true);
        //println!("{:?}", m);
    }
    part2_stacks.print_stacks(); // GMPMLWNMG
}
