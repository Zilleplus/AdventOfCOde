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

fn contains_duplicate(window: &Vec<char>) -> bool {
    for (i, c) in window.iter().enumerate() {
        let duplicate = window.iter().skip(i + 1).find(|x| x == &c);
        if let Some(_a) = duplicate {
            return true;
        }
    }

    false
}

fn find_first_valid_window(line: &String, window_size: usize)
{
    for i in 0..line.len() - window_size {
        let window: Vec<char> = line.chars().skip(i).take(window_size).collect();
        if !contains_duplicate(&window) {
            println!("found no duplicate it {:?} at i:{}", &window, i + window_size);
            break;
        }
        else{
            println!("found duplicate it {:?} at i:{}", &window, i + window_size);
        }
    }
}

fn main() {
    let p = Path::new("input.txt");
    let line = read_lines(p)
        .expect("unable to read input file input.txt")
        .nth(0)
        // We ignore io errors here, I also like to live dangerously.
        .unwrap()
        .unwrap();

    // find_first_valid_window(&line, 4);
    find_first_valid_window(&line, 14);
}
