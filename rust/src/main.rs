use crate::day_five::run as day_five;
use crate::day_four::run as day_four;
use crate::day_one::run as day_one;
use crate::day_seven::run as day_seven;
use crate::day_six::run as day_six;
use crate::day_three::run as day_three;
use crate::day_two::run as day_two;
use std::{fs, io};

mod day_five;
mod day_four;
mod day_one;
mod day_seven;
mod day_six;
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
        "4" => {
            day_four(&input);
        }
        "5" => {
            day_five(&input);
        }
        "6" => {
            println!(
                "Remember to space last column in your input by adding extra spaces to operator and lines with numbers of lower length!"
            );
            day_six(&input);
        }
        "7" => {
            day_seven(&input);
        }
        _ => {
            unimplemented!()
        }
    }
}
