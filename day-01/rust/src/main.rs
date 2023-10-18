use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("AOC Day 01");
    println!("===========");
    println!("Part 1");
    let now = Instant::now();

    let mut elf_totals = get_elf_totals();

    let max_elf = elf_totals.iter().max().unwrap();

    println!("Max Elf: {}", max_elf);
    println!("===========");
    let duration = now.elapsed();
    println!("Completed in {:?}", duration);
    println!("===========");

    let now2 = Instant::now();

    elf_totals.sort();
    elf_totals.reverse();

    let max_3_elves = elf_totals.iter().take(3).sum::<i32>();

    println!("Max 3 Elves: {}", max_3_elves);
    println!("===========");

    let duration2 = now2.elapsed();
    println!("Completed in {:?}", duration2);
    println!("===========");
}

fn get_elf_totals() -> Vec<i32> {
    let mut total: i32 = 0;
    let mut totals = Vec::new();
    let lines = read_lines("./input.txt").unwrap();
    for line in lines {
        match line {
            Ok(l) => {
                if l == "" {
                    totals.push(total);
                    total = 0;
                } else {
                    total += l.parse::<i32>().unwrap();
                }
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
    totals
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
