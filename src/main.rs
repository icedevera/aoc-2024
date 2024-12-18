use std::time::Instant;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let start = Instant::now();

    day1::part_1();
    day1::part_2();
    day2::part_1();
    day2::part_2();
    day3::part_1();
    day3::part_2();
    day4::part_1();
    day4::part_2();
    day5::part_1();
    day5::part_2();
    day6::part_1();
    day6::part_2();
    day7::part_1();
    day7::part_2();
    day8::part_1();
    day8::part_2();
    day9::part_1();
    day9::part_2();
    day10::part_1();
    day10::part_2();
    day11::part_1();
    day11::part_2_optimized();
    day12::part_1();
    day12::part_2();
    day13::part_1();
    day13::part_2();
    day14::part_1();
    day14::part_2();

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
