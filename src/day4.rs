use crate::solver::*;

/*
 * --- Day 4: Secure Container ---
 * You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had written the password on a sticky note, but someone threw it out.
 *
 * However, they do remember a few key facts about the password:
 *
 * It is a six-digit number.
 * The value is within the range given in your puzzle input.
 * Two adjacent digits are the same (like 22 in 122345).
 * Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
 * Other than the range rule, the following are true:
 *
 * 111111 meets these criteria (double 11, never decreases).
 * 223450 does not meet these criteria (decreasing pair of digits 50).
 * 123789 does not meet these criteria (no double).
 * How many different passwords within the range given in your puzzle input meet these criteria?
 */

pub struct Day4 {
    filename: &'static str,
}

impl Solver for Day4 {
    fn new(input_file: &'static str) -> Day4 {
        Day4 {
            filename: input_file,
        }
    }

    fn solve(&self) -> String {
        let f = std::fs::read_to_string(self.filename)
            .expect("cannot read file")
            .split('-')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        format!(
            "Solution part1 -> {}\n\tSolution part2 -> {}",
            self.part1(f[0], f[1]),
            99
        )
    }
}

impl Day4 {
    fn part1(&self, from: i64, to: i64) -> i64 {
        from + to
    }
}
