use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("AOC Day 4");
    println!("===========");
    println!("Part 1");
    let mut now = Instant::now();

    let cleaning_pairs = get_cleaning_pairs();

    let mut overlap_count = cleaning_pairs
        .iter()
        .filter(|cleaning_pair| cleaning_pair.has_total_consuming_area())
        .count();

    println!("Total: {:?}", overlap_count);

    let mut duration = now.elapsed();
    println!("Completed in {:?}", duration);
    println!("===========");
}

struct CleaningPairs {
    cleaner_1_areas: (i32, i32),
    cleaner_2_areas: (i32, i32),
}

impl CleaningPairs {
    fn new(cleaner_1_areas: (i32, i32), cleaner_2_areas: (i32, i32)) -> CleaningPairs {
        CleaningPairs {
            cleaner_1_areas,
            cleaner_2_areas,
        }
    }

    fn from(str: &str) -> CleaningPairs {
        let pair = str.split(",").collect::<Vec<&str>>();

        let cleaner_1_areas = pair[0].split("-").collect::<Vec<&str>>();
        let cleaner_2_areas = pair[1].split("-").collect::<Vec<&str>>();

        return CleaningPairs::new(
            (
                cleaner_1_areas[0].parse::<i32>().unwrap(),
                cleaner_1_areas[1].parse::<i32>().unwrap(),
            ),
            (
                cleaner_2_areas[0].parse::<i32>().unwrap(),
                cleaner_2_areas[1].parse::<i32>().unwrap(),
            ),
        );
    }

    fn has_total_consuming_area(&self) -> bool {
        let cleaner_1_areas = self.cleaner_1_areas;
        let cleaner_2_areas = self.cleaner_2_areas;

        let cleaner_1_start = cleaner_1_areas.0;
        let cleaner_1_end = cleaner_1_areas.1;

        let cleaner_2_start = cleaner_2_areas.0;
        let cleaner_2_end = cleaner_2_areas.1;

        let cleaner_1_consumes_the_other =
            cleaner_1_start <= cleaner_2_start && cleaner_1_end >= cleaner_2_end;
        let cleaner_2_consumes_the_other =
            cleaner_2_start <= cleaner_1_start && cleaner_2_end >= cleaner_1_end;

        return cleaner_1_consumes_the_other || cleaner_2_consumes_the_other;
    }
}

fn get_cleaning_pairs() -> Vec<CleaningPairs> {
    let mut cleaning_pairs: Vec<CleaningPairs> = Vec::new();

    let lines = read_lines("./input.txt").unwrap();
    for line in lines {
        match line {
            Ok(raw_line) => {
                let cleaning_pair = CleaningPairs::from(&raw_line);

                cleaning_pairs.push(cleaning_pair);
            }
            Err(_) => continue,
        }
    }

    return cleaning_pairs;
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
