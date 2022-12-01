use std::env;

mod day1;

fn main() {
    for arg in env::args().into_iter() {
        match arg.as_str() {
            "1" => day1::main(),
            _ => {}
        }
    }
}
