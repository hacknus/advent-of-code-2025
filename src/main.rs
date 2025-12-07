#[allow(unused)]
mod days;
mod io;
mod problem;
use colored::Colorize;
use copypasta::{ClipboardContext, ClipboardProvider};

use chrono::Datelike;
use days::day_1::DayOne;
use days::day_10::DayTen;
use days::day_11::DayEleven;
use days::day_12::DayTwelve;
use days::day_2::DayTwo;
use days::day_3::DayThree;
use days::day_4::DayFour;
use days::day_5::DayFive;
use days::day_6::DaySix;
use days::day_7::DaySeven;
use days::day_8::DayEight;
use days::day_9::DayNine;
use problem::Problem;
use std::time::Instant;

fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(DayOne {})),
        2 => Some(Box::new(DayTwo {})),
        3 => Some(Box::new(DayThree {})),
        4 => Some(Box::new(DayFour {})),
        5 => Some(Box::new(DayFive {})),
        6 => Some(Box::new(DaySix {})),
        7 => Some(Box::new(DaySeven {})),
        8 => Some(Box::new(DayEight {})),
        9 => Some(Box::new(DayNine {})),
        10 => Some(Box::new(DayTen {})),
        11 => Some(Box::new(DayEleven {})),
        12 => Some(Box::new(DayTwelve {})),
        _ => None,
    }
}

fn main() {
    let mut ctx = ClipboardContext::new().unwrap();

    let current_date = chrono::Utc::now();
    let day = current_date.day();
    match day_to_problem(day as usize) {
        None => {
            println!("No problem for day {day}...");
        }
        Some(problem) => {
            let start = Instant::now();
            let answer_one = problem.part_one(format!("input/puzzle_{day}.txt").as_str());
            println!("solving task one took {:?}", start.elapsed());

            let start = Instant::now();
            let answer_two = problem.part_two(format!("input/puzzle_{day}.txt").as_str());
            println!("solving task two took {:?}", start.elapsed());

            if answer_two == "no solution yet" {
                if ctx.get_contents().unwrap_or("".to_string()) == answer_one {
                    println!("{}", "\nAttention! Answer 1 is not a new answer!\n".red())
                }
                ctx.set_contents(answer_one.to_owned()).unwrap();
            } else {
                if ctx.get_contents().unwrap_or("".to_string()) == answer_two {
                    println!("{}", "\nAttention! Answer 2 is not a new answer!\n".red())
                }
                ctx.set_contents(answer_two.to_owned()).unwrap();
            }

            println!("Answer of Task Day {day}/1:");
            println!("{answer_one}\n");
            println!("Answer of Task Day {day}/2:");
            println!("{answer_two}");
        }
    }
}
