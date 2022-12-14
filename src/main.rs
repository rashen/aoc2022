use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    for arg in env::args().into_iter() {
        match arg.as_str() {
            "1" => day1::main(),
            "2" => day2::main(),
            "3" => day3::main(),
            "4" => day4::main(),
            "5" => day5::main(),
            "6" => day6::main(),
            _ => {}
        }
    }
}
