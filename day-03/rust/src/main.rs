use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("AOC Day 3");
    println!("===========");
    println!("Part 1");
    let mut now = Instant::now();

    let rucksacks = get_rucksacks();
    let rucksacks_by_priority_item: Vec<u32> = rucksacks
        .iter()
        .map(|ruck_sack| ruck_sack.common_item())
        .map(|common_item| get_priority_of_item(common_item))
        .collect();
    let total = rucksacks_by_priority_item.iter().sum::<u32>();

    println!("Total: {:?}", total);

    let mut duration = now.elapsed();
    println!("Completed in {:?}", duration);
    println!("===========");

    println!("Part 2");
    now = Instant::now();

    let mut current_low: i32 = 0;
    let mut current_high: i32 = 2;

    let mut common_item_priority_sum: u32 = 0;

    let mut ruck_sack_iterator = rucksacks.iter();

    loop {
        println!("Current low: {:?}", current_low);
        println!("Current high: {:?}", current_high);
        if current_high > rucksacks.len() as i32 {
            break;
        }

        let team_1_items = ruck_sack_iterator.next();
        let team_2_items = ruck_sack_iterator.next();
        let team_3_items = ruck_sack_iterator.next();

        if team_1_items.is_none() || team_2_items.is_none() || team_3_items.is_none() {
            break;
        }

        let team_1_items = team_1_items.unwrap().all_items();
        let team_2_items = team_2_items.unwrap().all_items();
        let team_3_items = team_3_items.unwrap().all_items();

        let mut common_items = team_1_items
            .iter()
            .filter(|&i| team_2_items.contains(i))
            .filter(|&i| team_3_items.contains(i))
            .map(|&i| i.clone());

        let common_item: Option<char> = common_items.next();

        let common_item_priority = get_priority_of_item(common_item);

        common_item_priority_sum += common_item_priority;

        current_low += 3;
        if current_high + 3 > (rucksacks.len() - 1) as i32 {
            current_high = (rucksacks.len() - 1) as i32;
        } else {
            current_high += 3;
        }
    }

    println!("Sum of priority items: {:?}", common_item_priority_sum);

    let duration = now.elapsed();
    println!("Completed in {:?}", duration);
    println!("===========");
}

fn get_rucksacks() -> Vec<RuckSack> {
    let mut rucksacks: Vec<RuckSack> = Vec::new();
    let lines = read_lines("./input.txt").unwrap();
    for line in lines {
        match line {
            Ok(raw_line) => {
                let half = raw_line.len() / 2;

                let ruck_sack = RuckSack {
                    compartment_1: raw_line[0..half].chars().collect(),
                    compartment_2: raw_line[half..].chars().collect(),
                };

                rucksacks.push(ruck_sack);
            }
            Err(_) => continue,
        }
    }

    rucksacks
}

#[derive(Debug)]
struct RuckSack {
    compartment_1: Vec<char>,
    compartment_2: Vec<char>,
}

impl RuckSack {
    // return the char that has only one occurence in both compartments
    fn common_item(&self) -> Option<char> {
        let mut common_item: char = ' ';

        for item in &self.compartment_1 {
            if self.compartment_2.contains(item) {
                let item_count = &self.compartment_1.iter().filter(|&i| i == item).count();

                match item_count {
                    i if i > &0 => {
                        common_item = *item;
                        break;
                    }
                    _ => continue,
                }
            }
        }

        match &common_item {
            ' ' => None,
            _ => Some(common_item),
        }
    }

    fn all_items(&self) -> Vec<&char> {
        let mut all_items: Vec<&char> = Vec::new();

        for item in &self.compartment_1 {
            all_items.push(item);
        }

        for item in &self.compartment_2 {
            all_items.push(item);
        }

        all_items
    }
}

fn get_priority_of_item(item: Option<char>) -> u32 {
    let item = item.unwrap_or(' ');
    let is_lower_case = item.is_ascii_lowercase();
    match is_lower_case {
        true => item.to_digit(36).unwrap_or(9) - 9,
        false => item.to_digit(36).unwrap_or(17) + 17,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_priority_of_item() {
        assert_eq!(get_priority_of_item(Some('a')), 1);
        assert_eq!(get_priority_of_item(Some('z')), 26);
        assert_eq!(get_priority_of_item(Some('A')), 27);
        assert_eq!(get_priority_of_item(Some('Z')), 52);
    }

    #[test]
    fn test_rucksack_common_item() {
        let ruck_sack = RuckSack {
            compartment_1: "abcdex".chars().collect(),
            compartment_2: "efghij".chars().collect(),
        };

        assert_eq!(ruck_sack.common_item(), Some('e'));
    }

    #[test]
    fn ruck_sack_1() {
        let ruck_sack = RuckSack {
            compartment_1: "vJrwpWtwJgWr".chars().collect(),
            compartment_2: "hcsFMMfFFhFp".chars().collect(),
        };

        assert_eq!(ruck_sack.common_item(), Some('p'));
    }

    #[test]
    fn ruck_sack_2() {
        let ruck_sack = RuckSack {
            compartment_1: "jqHRNqRjqzjGDLGL".chars().collect(),
            compartment_2: "rsFMfFZSrLrFZsSL".chars().collect(),
        };

        assert_eq!(ruck_sack.common_item(), Some('L'));
    }
}
