use std::collections::HashMap;

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
            self.part2(f[0], f[1])
        )
    }
}

impl Day4 {
    fn part1(&self, from: i64, to: i64) -> usize {
        (from..to)
            .into_iter()
            .filter(|&n| digit_increase(n) && has_digit_same(n))
            .count()
    }
}

fn has_digit_same(number: i64) -> bool {
    let mut map = HashMap::new();
    for c in number.to_string().chars() {
        if !map.contains_key(&c) {
            map.insert(c, 1);
            continue;
        }

        if let Some(x) = map.get_mut(&c) {
            *x = *x + 1;
        }
    }

    let mut c = map.iter().filter(|v| *v.1 > 1);
    c.next() != None
}

fn digit_increase(number: i64) -> bool {
    let mut last = 0;
    for n in number.to_string().chars() {
        if let Some(nn) = n.to_digit(10) {
            if nn >= last {
                last = nn;
            } else {
                return false;
            }
        }
    }

    true
}

/*
 * --- Part Two ---
 * An Elf just remembered one more important detail: the two adjacent matching digits are not part of a larger group of matching digits.
 *
 * Given this additional criterion, but still ignoring the range rule,
 * the following are now true:
 *
 * 112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
 * 123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
 * 111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
 * How many different passwords within the range given in your puzzle input meet all of the criteria?
 */

impl Day4 {
    fn part2(&self, from: i64, to: i64) -> usize {
        (from..to)
            .into_iter()
            .filter(|&n| digit_increase(n) && has_adjacent_same_digit(n))
            .count()
    }
}

fn has_adjacent_same_digit(number: i64) -> bool {
    let x: Vec<_> = format!("{}", number).chars().collect();
    let mut same = HashMap::<char, usize>::new();
    for w in x.windows(2) {
        let a = w[0];
        let b = w[1];
        if a == b {
            *same.entry(a).or_default() += 1;
        }
    }
    same.iter().filter(|x| *x.1 == 1).count() > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation() {
        assert_eq!(has_digit_same(122345), true);
        assert_eq!(has_digit_same(123456), false);
        assert_eq!(has_adjacent_same_digit(112233), true);
        assert_eq!(has_adjacent_same_digit(123444), false);
        assert_eq!(has_adjacent_same_digit(111122), true);
        assert_eq!(has_adjacent_same_digit(111241), false);

        assert_eq!(digit_increase(123456), true);
        assert_eq!(digit_increase(111111), true);
        assert_eq!(digit_increase(654321), false);

        let d = Day4::new("");
        assert_eq!(d.part1(165432, 707912), 1716);
        assert_eq!(d.part2(165432, 707912), 1163);
    }
}
