use std::env;

mod day1;
mod day2;
mod day3;

fn main() {
    for arg in env::args().into_iter() {
        match arg.as_str() {
            "1" => day1::main(),
            "2" => day2::main(),
            "3" => day3::main(),
            _ => {}
        }
    }
}
