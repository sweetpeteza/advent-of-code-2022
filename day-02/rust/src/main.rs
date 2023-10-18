use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("AOC Day 02");
    println!("===========");
    println!("Part 1");
    let now = Instant::now();

    let matches = get_match_results_incomplete_instructions();
    let total_score = matches.iter().sum::<i32>();

    println!("Total score: {}", total_score);

    let duration = now.elapsed();
    println!("Completed in {:?}", duration);
    println!("===========");
    println!("Part 2");
    let now2 = Instant::now();

    let matches = get_match_results();
    let total_score = matches.iter().sum::<i32>();

    println!("Total score: {}", total_score);
    println!("Completed in {:?}", now2.elapsed());
    println!("===========");
}

fn get_match_results() -> Vec<i32> {
    let mut matches = Vec::new();
    let lines = read_lines("./input.txt").unwrap();
    for line in lines {
        match line {
            Ok(l) => {
                let mut split = l.split(" ");
                let opponent_hand = split.next().unwrap();
                let opponent_hand = get_opponent_hand(&opponent_hand);
                let proposed_result = split.next().unwrap();
                let proposed_result = get_round_result(&proposed_result);

                let player_hand = get_hand_to_match_result(&opponent_hand, &proposed_result);
                let match_score = get_match_score(&player_hand, &opponent_hand);

                matches.push(match_score);
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
    matches
}

fn get_match_results_incomplete_instructions() -> Vec<i32> {
    let mut matches = Vec::new();
    let lines = read_lines("./input.txt").unwrap();
    for line in lines {
        match line {
            Ok(l) => {
                let mut split = l.split(" ");
                let opponent_hand = split.next().unwrap();
                let player_hand = split.next().unwrap();

                let player_hand = get_player_hand(&player_hand);
                let opponent_hand = get_opponent_hand(&opponent_hand);
                let match_score = get_match_score(&player_hand, &opponent_hand);

                matches.push(match_score);
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
    matches
}

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum RoundResult {
    Win,
    Lose,
    Draw,
}

const WIN_POINTS: i32 = 6;
const DRAW_POINTS: i32 = 3;
const LOSE_POINTS: i32 = 0;

fn get_opponent_hand(hand: &str) -> Hand {
    match hand {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissors,
        _ => panic!("Invalid hand"),
    }
}
fn get_player_hand(hand: &str) -> Hand {
    match hand {
        "X" => Hand::Rock,
        "Y" => Hand::Paper,
        "Z" => Hand::Scissors,
        _ => panic!("Invalid hand"),
    }
}

fn get_round_result(hand: &str) -> RoundResult {
    match hand {
        "X" => RoundResult::Lose,
        "Y" => RoundResult::Draw,
        "Z" => RoundResult::Win,
        _ => panic!("Unknown result"),
    }
}

fn get_hand_to_match_result(opponent_hand: &Hand, proposed_result: &RoundResult) -> Hand {
    match opponent_hand {
        Hand::Rock => match proposed_result {
            RoundResult::Win => Hand::Paper,
            RoundResult::Draw => Hand::Rock,
            RoundResult::Lose => Hand::Scissors,
        },
        Hand::Paper => match proposed_result {
            RoundResult::Win => Hand::Scissors,
            RoundResult::Draw => Hand::Paper,
            RoundResult::Lose => Hand::Rock,
        },
        Hand::Scissors => match proposed_result {
            RoundResult::Win => Hand::Rock,
            RoundResult::Draw => Hand::Scissors,
            RoundResult::Lose => Hand::Paper,
        },
    }
}

fn get_match_score(player_hand: &Hand, opponent_hand: &Hand) -> i32 {
    let player_hand_score = match player_hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    };

    let round_result = match opponent_hand {
        Hand::Rock => match player_hand {
            Hand::Rock => RoundResult::Draw,
            Hand::Paper => RoundResult::Win,
            Hand::Scissors => RoundResult::Lose,
        },
        Hand::Paper => match player_hand {
            Hand::Rock => RoundResult::Lose,
            Hand::Paper => RoundResult::Draw,
            Hand::Scissors => RoundResult::Win,
        },
        Hand::Scissors => match player_hand {
            Hand::Rock => RoundResult::Win,
            Hand::Paper => RoundResult::Lose,
            Hand::Scissors => RoundResult::Draw,
        },
    };

    match round_result {
        RoundResult::Win => player_hand_score + WIN_POINTS,
        RoundResult::Draw => player_hand_score + DRAW_POINTS,
        RoundResult::Lose => player_hand_score + LOSE_POINTS,
    }
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
