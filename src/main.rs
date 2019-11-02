mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day10;
mod day11;
mod day12;
mod day17;
mod day23;
mod util;

fn main() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
	return Err("Please give day".into());
    }
    let day = args[1].parse::<usize>().expect("Malformed day number");
    match day {
	1 => day1::run(),
	2 => day2::run(),
	3 => day3::run(),
	4 => day4::run(),
	5 => day5::run(),
	6 => day6::run(),
	7 => day7::run(),
	10 => day10::run(),
	11 => day11::run(),
	12 => day12::run(),
	17 => day17::run(),
	23 => day23::run(),
	_ => return Err(format!("Unknown day {}", day)),
    }
    return Ok(());
}
