//use clap::{Arg, App, SubCommand};
use clap::Parser;
use std::borrow::Borrow;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: std::path::PathBuf,
}

fn read_lines(filename: &std::path::Path) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    Ok(lines)
}

fn main() {
    let args = Args::parse();

    if !args.path.exists() {
        let pathstring =
            args.path.into_os_string().into_string().expect(
                "Invalid filepath, the path doesn't exist, and is not convertable to string",
            );
        panic!(".filepath {pathstring} does not exist.");
    }

    if let Ok(lines) = read_lines(&args.path.as_path()) {
        let mut elves_cal = Vec::new();
        let mut total_cal_elf = 0;
        for (line_index, line) in lines.enumerate() {
            if let Ok(l) = line {
                if l == "" {
                    // empty line -> next elf
                    let id = elves_cal.len();
                    println!("Added elf(id={id}) with {total_cal_elf} calories");
                    elves_cal.push((total_cal_elf, id));
                    total_cal_elf = 0;
                } else {
                    if let Ok(cal) = l.parse::<i32>() {
                        total_cal_elf = total_cal_elf + cal;
                    } else {
                        panic!("unable to parse string to int a line {line_index}")
                    }
                }
            }
        }

        let (max_val, max_id) = (&elves_cal)
            .iter()
            .max_by_key(|(val, _index)| val)
            .expect("No elfs found in the file");
        println!("The elf with the most calories: id={max_id} with {max_val} calories");

        elves_cal.sort_by_key(|(val, _index)| -val.clone());
        let top_3_elves = elves_cal.iter().take(3);

        // This somehow ignores the end of line, not sure why.
        let top3_string_elves = top_3_elves.clone()
            .map(|(val, index)| format!("elf val={val} index={index}"))
            .fold(String::new(), |elve, message: String| {
                message + " " + elve.borrow() + " \r\n"
            });

        print!("top 3 elves: \n\r {top3_string_elves}");

        let top3_cals = top_3_elves.fold(0, |acc, (val, index_)| acc + val);
        println!("The top 3 cals={top3_cals}");
    } else {
        // TODO provide a proper error here.
        panic!("Unable to read file");
    }
}
