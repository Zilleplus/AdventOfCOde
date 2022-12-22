use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines}, path::Path
};

fn read_lines(filename: &std::path::Path) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    Ok(lines)
}

// these 2 functions can be merged into a common one.
fn find_common_char(left: &str, right: &str) -> char {
    left.chars().find(|&c| {return right.contains(c)})
        .expect("No common character found between the two strings!")
}
fn find_common_char3(left: &str, middle: &str, right: &str) -> char
{
    let common: Vec<char> = left.chars()
        .filter(|x| middle.contains(x.clone()))
        .filter(|x|right.contains(x.clone()))
        .collect();
        
    if  common.len() == 0 {
        panic!("no common chars found in group");
    }

    return common[0]
}

fn encode(c: char) -> u8{
    let mut buff : [u8; 4] = [0,0,0,0];
    c.encode_utf8(&mut buff); // 97
    return buff[0];
}

fn character_score(input: char) -> i32 {
    let a = encode('a'); // 97
    let z = encode('z'); // 122
    let A = encode('A'); // 65
    let Z = encode('Z'); // 90

    let input = encode(input);

    if input >= a && input <= z{
        // a-z have priority 1-26
        return (input-a+1) as i32;
    }
    else if input >= A && input <= Z{
        // a-z have priority 27-52
        return ((input-A) + 27) as i32;
    }
    panic!("invalid character provided in character_score(...) function");
}

fn part1(lines: Vec<String>)
{
    let split_lines = lines.iter()
        .map(|x| {
            let half_length = x.len()/2;
            let (left, right) =  x.split_at(half_length);
            return (left, right);
        });

    let priorities = split_lines
        .map(|(left , right)| {return find_common_char(left, right)})
        .map(|x: char| character_score(x)) ;
    
    let priorities_sum: i32 = priorities.sum();


    println!("part1: The sum of the priorities={priorities_sum}"); // 7701
}

fn part2(lines: Vec<String>)
{
    let num_groups = lines.len()/3;
    let priorities = (0..num_groups).map(|i| {
        let left = lines[i*3].as_str();
        let middle = lines[i*3 + 1].as_str();
        let right = lines[i*3 + 2].as_str();
        return find_common_char3(left, middle, right);
    }).map(character_score);

    let total_score: i32 = priorities.sum();

    println!("part2: The sum of all triple groups is {total_score}");
}

fn main() {
    let p = Path::new("input.txt");
    let lines: Vec<String> = read_lines(p)
        .expect("unable to read input file")
        // This filter and map should be a monadic bind but no idea how to do this in rust.
        .filter(|x|x.is_ok())
        .map(|x|x.unwrap())
        .collect();

    // The iterators always take ownership, and the temporary has to outlive the first expression 
    // expressions. So sadly we have to clone here. (as far as I know)
    part1(lines.clone());
    part2(lines.clone());
}

#[cfg(test)]
mod tests{
    use crate::*;

    #[test]
    fn test_character_score(){
        let char_c = 'c';
        let expected_score_c: i32 = 3;
        let score_c = character_score(char_c);
        assert_eq!(expected_score_c, score_c);

        let char_capital_c = 'C';
        let expected_score_capital_c = 29;
        let score_capital_c = character_score(char_capital_c);
        assert_eq!(expected_score_capital_c, score_capital_c);
    }
}