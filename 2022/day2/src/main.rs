use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};
#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn read_lines(filename: &std::path::Path) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    Ok(lines)
}

fn map_symbol(symbol: &str, line_number: usize) -> Move {
    match symbol {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        _ => panic!("enable to map symbol='{symbol}' at line={line_number}"),
    }
}

fn map_result(symbol: &str, line_number: usize) -> GameResult {
    match symbol {
        "X" => GameResult::Lose,
        "Y" => GameResult::Draw,
        "Z" => GameResult::Win,
        _ => panic!("enable to map gameresult='{symbol}' at line={line_number}"),
    }
}

fn point_symbol(me: Move) -> i32 {
    return match me {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };
}

fn score(opponent: Move, me: Move) -> i32 {
    let points_move_choice = point_symbol(me);
    let match_points = match (opponent, me) {
        (Move::Rock, Move::Paper) => 6,     // win
        (Move::Paper, Move::Scissors) => 6, // win
        (Move::Scissors, Move::Rock) => 6,  // win
        (Move::Paper, Move::Rock) => 0,     // lose
        (Move::Scissors, Move::Paper) => 0, // lose
        (Move::Rock, Move::Scissors) => 0,  // lose
        _ => 3,                             // draw
    };

    return points_move_choice + match_points;
}

enum GameResult {
    Win,
    Draw,
    Lose
}

fn find_move(opponent: Move, wanted_result: GameResult) -> Move
{
    return match (opponent, wanted_result) {
        (Move::Rock, GameResult::Win) => Move::Paper,
        (Move::Rock, GameResult::Lose) => Move::Scissors,
        (Move::Paper, GameResult::Win) => Move::Scissors,
        (Move::Paper, GameResult::Lose) => Move::Rock,
        (Move::Scissors, GameResult::Win) => Move::Rock,
        (Move::Scissors, GameResult::Lose) => Move::Paper,
        _ => opponent // draw
    }
}

fn main() {
    let p = std::path::Path::new("./input.txt");

    let mut total_score_part1 = 0;
    let mut total_score_part2 = 0;
    for (line_index, maybe_line) in read_lines(p).expect("unable to read file").enumerate() {
        if let Ok(line) = maybe_line {
            // -> parse line
            let unmapped_symbols: Vec<&str> = line
                .split(" ")
                .collect();

            if unmapped_symbols.len() != 2 {
                panic!("line={line_index} does not contain 2 valid symbols.");
            }

            // Very ugly side effect in map symbol, could be better using result.
            let opponent_move = map_symbol(unmapped_symbols[0], line_index);
            let me_move = map_symbol(unmapped_symbols[1], line_index);

            let score_part1 = score(opponent_move, me_move);
            total_score_part1 = total_score_part1 + score_part1;
        
            // part2
            let wanted_result_part2 = map_result(unmapped_symbols[1], line_index);
            let wanted_move = find_move(opponent_move, wanted_result_part2);
            let score_part2 = score(opponent_move, wanted_move);
            total_score_part2 = total_score_part2 + score_part2;
        } else {
            panic!("unable to read line at {line_index}");
        }
    }

    print!("part1 score = {total_score_part1} -> should be 11841");
    print!("part2 score = {total_score_part2}'");
}
