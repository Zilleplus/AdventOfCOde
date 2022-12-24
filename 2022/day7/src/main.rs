use std::{
    fs,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn read_lines(filename: &std::path::Path) -> io::Result<Lines<BufReader<fs::File>>> {
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    Ok(lines)
}

pub trait Sizable {
    fn size(&self) -> i32;
}

#[derive(Debug, PartialEq, Eq)]
struct File {
    name: String,
    size: i32,
}

impl File {
    fn new(name: String, size: i32) -> File {
        File { name, size }
    }
}

impl Sizable for File {
    fn size(&self) -> i32 {
        self.size
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Directory {
    name: String,
    subdirs: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name,
            subdirs: Vec::new(),
            files: Vec::new(),
        }
    }
}

fn total_size<T>(elements: &Vec<T>) -> i32
where
    T: Sizable,
{
    elements
        .iter()
        .map(|x| x.size())
        .reduce(|x, y| x + y)
        .unwrap_or(0)
}

impl Sizable for Directory {
    fn size(&self) -> i32 {
        total_size(&self.files) + total_size(&self.subdirs)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Token<'a> {
    CD(&'a str), // location to chance dir to
    LS(),
    DIR(&'a str),       // name
    FILE(&'a str, i32), // name, size
}

fn parse_dir(line: &str) -> Token {
    let words = line.split(' ');
    Token::DIR(words.last().expect("invalid lines with directory"))
}

fn parse_file(line: &str) -> Token {
    let mut words = line.split(' ');

    let size = words.next().expect("no size on line with file");
    let size: i32 = size.parse().unwrap();
    let name = words.next().expect("no name on line with file");

    Token::FILE(name, size)
}

fn parse_command(line: &str) -> Token {
    let mut words = line.split(' ').skip(1);
    let command = words.next().expect("can't find command");
    match command {
        "ls" => Token::LS(),
        "cd" => {
            let cd_location = words.next().expect("can't find location cd");
            Token::CD(cd_location)
        }
        _ => panic!(""),
    }
}

#[derive(Debug)]
struct FileSystem {
    dirs: Vec<Directory>,
}

impl FileSystem {
    fn new() -> FileSystem {
        let root = Directory::new("/".to_owned());
        FileSystem { dirs: vec![root] }
    }

    fn change_dir_up(&mut self) {
        assert!(!self.dirs.is_empty());
        if let Some(child) = self.dirs.pop() {
            // We are already at the root, popping off a second
            // element will have no element. Makes sense as we
            // should do nothing.
            if let Some(mut current_dir) = self.dirs.pop() {
                current_dir.subdirs.push(child);
                self.dirs.push(current_dir); // put the current dir back
            }
        }
    }

    fn change_dir_root(&mut self) {
        assert!(!self.dirs.is_empty());
        // Keep going up untill everything is back into 1 node.
        while self.dirs.len() != 1 {
            self.change_dir_up();
        }
    }

    fn change_dir(&mut self, name: &str) {
        if name == ".." {
            self.change_dir_up();
        } else if name == "/" {
            self.change_dir_root();
        } else {
            // remove the dir from the list of dirs
            if let Some(mut current_dir) = self.dirs.pop() {
                if let Some(new_current_dir_index) = current_dir
                    .subdirs
                    .iter()
                    .enumerate()
                    .find(|(_i, dir)| dir.name == name)
                    .map(|(i, _dir)| i)
                {
                    let new_change_dir = current_dir.subdirs.remove(new_current_dir_index);

                    self.dirs.push(current_dir);
                    self.dirs.push(new_change_dir);
                } else {
                    panic!("Unable to find subfolder {:?}.", name);
                }
            }
        }
    }

    // add directory to current dir
    fn push_directory(&mut self, name: &str) {
        let new_dir = Directory::new(name.to_owned());
        if let Some(current_dir) = self.dirs.iter_mut().last() {
            (&mut current_dir.subdirs).push(new_dir);
        }
    }

    // add file to current directory
    fn push_file(&mut self, name: &str, size: i32) {
        let new_file = File::new(name.to_owned(), size.clone());
        if let Some(current_dir) = self.dirs.iter_mut().last() {
            (&mut current_dir.files).push(new_file);
        }
    }
}

struct Parser<'a> {
    fs: FileSystem,
    tokens: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token>) -> Parser {
        let mut tokens = tokens;
        tokens.reverse();

        Parser {
            fs: FileSystem::new(),
            tokens,
        }
    }

    fn next(&mut self) -> Option<Token<'a>> {
        self.tokens.pop()
    }

    fn take_files_or_dirs(&mut self) {
        loop {
            if let Some(t) = self.next() {
                match t {
                    Token::DIR(name) => {
                        self.fs.push_directory(name);
                    }
                    Token::FILE(name, size) => {
                        self.fs.push_file(name, size.clone());
                    }
                    _ => {
                        // Not the correct kind of toke, put it back.
                        self.tokens.push(t);
                        break;
                    }
                }
            } else {
                // End of the file stop.
                break;
            }
        }
    }

    fn parse(&mut self) {
        while let Some(token) = self.next() {
            match token {
                Token::CD(name) => self.fs.change_dir(name),
                Token::LS() => self.take_files_or_dirs(),
                _ => panic!("Unsuppored token"),
            }
        }

        self.fs.change_dir_root();
    }
}

trait MutableDirectoryVisitor {
    // Returns true if continue, false if stop.
    fn visit(&mut self, dir: &Directory);
}

struct DirectoryCounter {
    count: i32,
}

impl MutableDirectoryVisitor for DirectoryCounter {
    fn visit(&mut self, dir: &Directory) {
        self.count = self.count + 1;
    }
}

fn count_dirs(fs: &FileSystem) -> i32 {
    let mut vis = DirectoryCounter { count: 0 };
    visit_mutable_directory_visitor(&fs, &mut vis);

    vis.count
}

fn visit_mutable_directory_visitor<TVisitor>(fs: &FileSystem, vis: &mut TVisitor)
where
    TVisitor: MutableDirectoryVisitor,
{
    let mut visits: Vec<&Directory> = Vec::new();

    for f in &fs.dirs {
        visits.push(f);
    }

    while let Some(top_el) = visits.pop() {
        for f in &top_el.subdirs {
            visits.push(f);
        }
        vis.visit(&top_el);
    }
}

#[derive(Debug)]
struct Part1Counter {
    number_of_files: i32,
    total_size: i32,
    dir_names: Vec<String>,
}

impl Part1Counter {
    fn new() -> Part1Counter {
        Part1Counter {
            number_of_files: 0,
            total_size: 0,
            dir_names: Vec::new(),
        }
    }
}

impl MutableDirectoryVisitor for Part1Counter {
    fn visit(&mut self, dir: &Directory) {
        let s = dir.size();
        println!("Analyzing directory name={:?} size={:?}.", &dir.name, &s);
        if s <= 100000 {
            // "at most"
            self.number_of_files = self.number_of_files + 1;
            self.total_size = self.total_size + s;
            self.dir_names.push(dir.name.clone());
        }
    }
}

struct Part2Vis{
    required_free_size: i32,
    solution: Option<(String, i32)>
}

impl MutableDirectoryVisitor for Part2Vis {
    fn visit(&mut self, dir: &Directory) {
        let s = dir.size();
        if s > self.required_free_size {
            if let Some((_, current_solution_size)) = self.solution{
                if current_solution_size < s {
                    // Only accept better fitting solutions.
                    self.solution = Some((dir.name.clone(), s));
                }
            }
            else{
                // No solution was found so far, so use the first fitting one.
                self.solution = Some((dir.name.clone(), s));
            }
        }
    }
}

struct DirsVistor {
    dirs: Vec<String>,
}

impl MutableDirectoryVisitor for DirsVistor {
    fn visit(&mut self, dir: &Directory) {
        self.dirs.push(dir.name.clone());
    }
}

fn main() {
    let p = Path::new("input.txt");
    let lines: Vec<String> = read_lines(p)
        .expect("unable to read input file input.txt")
        .map(|x| x.unwrap())
        .skip(1) // we ignore the first line, as it's a cd to the root
        .collect();

    // step1: read out the file, and populate the tree structure
    let tokens: Vec<Token> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let first_char = line
                .chars()
                .nth(0)
                .unwrap_or_else(|| panic!("Missing first character at line={i}."));
            match first_char {
                'd' => parse_dir(line),
                '$' => parse_command(line),
                '1'..='9' => parse_file(line),
                _ => Token::LS(),
            }
        })
        .collect();

    // step2: To begin, find all of the directories with a total size of at most 100000,
    // then calculate the sum of their total sizes.
    let mut p = Parser::new(tokens);
    p.parse();

    let mut part1_visitor = Part1Counter::new();
    visit_mutable_directory_visitor(&p.fs, &mut part1_visitor);
    println!("The total size={:?}.", &part1_visitor.total_size);
}

#[cfg(test)]
mod tests {
    use crate::{parse_dir, parse_file, FileSystem, Token};

    #[test]
    fn test_dir_parse() {
        let input = "dir qcznqph";
        let res_dir_token = parse_dir(input);

        let expected_dir_name = "qcznqph";
        let expected_dir_token = Token::DIR(expected_dir_name);

        assert!(res_dir_token == expected_dir_token);
    }

    #[test]
    fn test_file_parse() {
        let input = "184686 jzn";
        let res_dir_token = parse_file(input);

        let expected_file_name = "jzn";
        let expected_file_size = 184686;
        let expected_dir_token = Token::FILE(expected_file_name, expected_file_size);

        assert!(res_dir_token == expected_dir_token);
    }

    #[test]
    fn test_filesystem() {
        let mut fs = FileSystem::new();
        fs.push_file("file1", 1);
        fs.push_directory("dir1");
        fs.push_directory("dir2");
        fs.change_dir("dir2");
        fs.push_file("dir2_file1", 3);
        fs.push_file("dir2_file2", 4);

        fs.change_dir_up();
        fs.change_dir("dir2_file1");

        fs.change_dir("/");

        assert!(fs.dirs.len() == 1);
        let root = fs.dirs.iter().last().unwrap();
        assert!(root.name == "/");
        assert!(root.subdirs.len() == 2);
        let dir1 = root.subdirs.get(0).unwrap();
        assert!(dir1.name == "dir1");
        let dir2 = root.subdirs.get(1).unwrap();
        assert!(dir2.name == "dir2");

        let dir2_file1 = dir2.files.get(0).unwrap();
        assert!(dir2_file1.name == "dir2_file1");
        assert!(dir2_file1.size == 3);
        let dir2_file2 = dir2.files.get(1).unwrap();
        assert!(dir2_file2.name == "dir2_file2");
        assert!(dir2_file2.size == 4);
    }
}
