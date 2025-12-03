use crate::day_one::run as day_one;
use crate::day_three::run as day_three;
use crate::day_two::run as day_two;
use std::{fs, io};

mod day_one;
mod day_three;
mod day_two;

fn main() {
    let mut day_number = String::new();

    println!("Enter day number: ");

    io::stdin()
        .read_line(&mut day_number)
        .expect("Failed to read line");

    let day_number = day_number.trim();

    let input = fs::read_to_string(format!("./inputs/{day_number}")).unwrap();

    match day_number {
        "1" => {
            day_one(&input);
        }
        "2" => {
            day_two(&input);
        }
        "3" => {
            day_three(&input);
        }
        _ => {
            unimplemented!()
        }
    }
}
