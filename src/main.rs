mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod intcode;
mod solver;

use solver::*;

fn main() {
    println!("Advent of code 2019!");

    let d = day1::Day1::new("input/day1");
    println!("Day1 -> {}", d.solve());

    let d = day2::Day2::new("input/day2");
    println!("Day2 -> {}", d.solve());

    let d = day3::Day3::new("input/day3");
    println!("Day3 -> {}", d.solve());

    let d = day4::Day4::new("input/day4");
    println!("Day4 -> {}", d.solve());

    let d = day5::Day5::new("input/day5");
    println!("Day5 -> {}", d.solve());

    let d = day6::Day6::new("input/day6");
    println!("Day6 -> {}", d.solve());

    let d = day7::Day7::new("input/day7");
    println!("Day7 -> {}", d.solve());

    let d = day8::Day8::new("input/day8");
    println!("Day8 -> {}", d.solve());
}
