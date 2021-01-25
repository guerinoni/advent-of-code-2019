use std::collections::HashMap;

use crate::solver::*;

/*
 * --- Day 6: Universal Orbit Map ---
 * You've landed at the Universal Orbit Map facility on Mercury. Because navigation in space often involves transferring between orbits,
 * the orbit maps here are useful for finding efficient routes between, for example, you and Santa. You download a map of the local orbits (your puzzle input).
 *
 * Except for the universal Center of Mass (COM), every object in space is in orbit around exactly one other object. An orbit looks roughly like this:
 *
 *                   \
 *                    \
 *                     |
 *                     |
 * AAA--> o            o <--BBB
 *                     |
 *                     |
 *                    /
 *                   /
 * In this diagram, the object BBB is in orbit around AAA. The path that BBB takes around AAA (drawn with lines) is only partly shown.
 * In the map data, this orbital relationship is written AAA)BBB, which means "BBB is in orbit around AAA".
 *
 * Before you use your map data to plot a course, you need to make sure it wasn't corrupted during the download.
 * To verify maps, the Universal Orbit Map facility uses orbit count checksums - the total number of direct orbits (like the one shown above) and indirect orbits.
 *
 * Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain can be any number of objects long:
 * if A orbits B, B orbits C, and C orbits D, then A indirectly orbits D.
 *
 * For example, suppose you have the following map:
 *
 * COM)B
 * B)C
 * C)D
 * D)E
 * E)F
 * B)G
 * G)H
 * D)I
 * E)J
 * J)K
 * K)L
 * Visually, the above map of orbits looks like this:
 *
 *         G - H       J - K - L
 *        /           /
 * COM - B - C - D - E - F
 *                \
 *                 I
 * In this visual representation, when two objects are connected by a line, the one on the right directly orbits the one on the left.
 *
 * Here, we can count the total number of orbits as follows:
 *
 * D directly orbits C and indirectly orbits B and COM, a total of 3 orbits.
 * L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a total of 7 orbits.
 * COM orbits nothing.
 * The total number of direct and indirect orbits in this example is 42.
 *
 * What is the total number of direct and indirect orbits in your map data?
 */

pub struct Day6 {
    filename: &'static str,
}

impl Solver for Day6 {
    fn new(input_file: &'static str) -> Day6 {
        Day6 {
            filename: input_file,
        }
    }

    fn solve(&self) -> String {
        let data = file_to_vec_of_string(self.filename);
        format!(
            "Solution part1 -> {}\n\tSolution part2 -> {}",
            self.part1(&data),
            self.part2(&data),
        )
    }
}

impl Day6 {
    fn part1(&self, data: &[String]) -> i64 {
        let map = data
            .iter()
            .map(|d| {
                let mut i = d.split(')').rev();
                (i.next().unwrap().to_string(), i.next().unwrap().to_string())
            })
            .collect::<HashMap<String, String>>();

        let mut sum = 0;
        for item in map.keys() {
            sum += count_from(&map, item);
        }

        sum
    }
}

fn count_from(map: &HashMap<String, String>, start: &str) -> i64 {
    let f = map.get(start);
    match f {
        Some(value) => 1 + count_from(map, value),
        None => 0,
    }
}

/*
 * --- Part Two ---
 * Now, you just need to figure out how many orbital transfers you (YOU) need to take to get to Santa (SAN).
 *
 * You start at the object YOU are orbiting; your destination is the object SAN is orbiting.
 * An orbital transfer lets you move from any object to an object orbiting or orbited by that object.
 *
 * For example, suppose you have the following map:
 *
 * COM)B
 * B)C
 * C)D
 * D)E
 * E)F
 * B)G
 * G)H
 * D)I
 * E)J
 * J)K
 * K)L
 * K)YOU
 * I)SAN
 * Visually, the above map of orbits looks like this:
 *
 *                           YOU
 *                          /
 *         G - H       J - K - L
 *        /           /
 * COM - B - C - D - E - F
 *                \
 *                 I - SAN
 * In this example, YOU are in orbit around K, and SAN is in orbit around I. To move from K to I, a minimum of 4 orbital transfers are required:
 *
 * K to J
 * J to E
 * E to D
 * D to I
 * Afterward, the map of orbits looks like this:
 *
 *         G - H       J - K - L
 *        /           /
 * COM - B - C - D - E - F
 *                \
 *                 I - SAN
 *                  \
 *                   YOU
 * What is the minimum number of orbital transfers required to move from the object YOU are orbiting to the object SAN is orbiting?
 * (Between the objects they are orbiting - not between YOU and SAN.)
*/

impl Day6 {
    fn part2(&self, data: &[String]) -> usize {
        let map = data
            .iter()
            .map(|d| {
                let mut i = d.split(')').rev();
                (i.next().unwrap().to_string(), i.next().unwrap().to_string())
            })
            .collect::<HashMap<String, String>>();

        let v1 = all_parents(&String::from("YOU"), &map);
        let v2 = all_parents(&String::from("SAN"), &map);

        for v in v1.iter().enumerate() {
            if v2.contains(v.1) {
                let i = v2.iter().enumerate().find(|item| item.1 == v.1).unwrap();
                return i.0 + v.0;
            }
        }

        0
    }
}

fn all_parents(start: &String, map: &HashMap<String, String>) -> Vec<String> {
    let mut all = Vec::new();
    let mut current = map.get(start);

    while let Some(p) = current {
        all.push(p.clone());
        current = map.get(p);
    }

    all
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn validate() {
        let d = Day6::new("");

        let intput = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<_>>();

        assert_eq!(d.part1(&intput), 42);

        let data = file_to_vec_of_string("input/day6");
        assert_eq!(d.part1(&data), 453028);

        let input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ]
        .iter()
        .map(|&t| t.to_string())
        .collect::<Vec<_>>();

        assert_eq!(d.part2(&input), 4);
    }
}
