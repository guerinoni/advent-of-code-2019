use crate::solver::*;

/*
 * --- Day 8: Space Image Format ---
 * The Elves' spirits are lifted when they realize you have an opportunity to reboot one of their
 * Mars rovers, and so they are curious if you would spend a brief sojourn on Mars. You land your ship near the rover.

 * When you reach the rover, you discover that it's already in the process of rebooting!
 * It's just waiting for someone to enter a BIOS password. The Elf responsible for the rover takes a picture
 * of the password (your puzzle input) and sends it to you via the Digital Sending Network.

 * Unfortunately, images sent via the Digital Sending Network aren't encoded with any normal encoding;
 * instead, they're encoded in a special Space Image Format. None of the Elves seem to remember why this is the case.
 * They send you the instructions to decode it.

 * Images are sent as a series of digits that each represent the color of a single pixel.
 * The digits fill each row of the image left-to-right, then move downward to the next row,
 * filling rows top-to-bottom until every pixel of the image is filled.

 * Each image actually consists of a series of identically-sized layers that are filled in this way.
 * So, the first digit corresponds to the top-left pixel of the first layer, the second digit corresponds
 * to the pixel to the right of that on the same layer, and so on until the last digit, which corresponds to the bottom-right pixel of the last layer.

 * For example, given an image 3 pixels wide and 2 pixels tall, the image data 123456789012 corresponds to the following image layers:

 * Layer 1: 123
 *          456

 * Layer 2: 789
 *          012
 * The image you received is 25 pixels wide and 6 pixels tall.

 * To make sure the image wasn't corrupted during transmission, the Elves would like you to find the layer
 * that contains the fewest 0 digits. On that layer, what is the number of 1 digits multiplied by the number of 2 digits?
*/

pub struct Day8 {
    filename: &'static str,
}

impl Solver for Day8 {
    fn new(input_file: &'static str) -> Day8 {
        Day8 {
            filename: input_file,
        }
    }

    fn solve(&self) -> String {
        let data = std::fs::read_to_string(self.filename).unwrap();
        format!(
            "Solution part1 -> {}\n\tSolution part2 -> {}",
            self.part1(data, 25, 6),
            0
        )
    }
}

fn decode_layer(data: String, w: usize, h: usize) -> Vec<Vec<String>> {
    let mut layers = Vec::new();
    for i in data
        .chars()
        .collect::<Vec<char>>()
        .chunks(w)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .chunks(h)
    {
        layers.push(i.to_vec());
    }

    layers
}

fn find_layer_with_fewest(layers: Vec<Vec<String>>, digit: i64) -> Vec<String> {
    let mut ret = Vec::new();
    let mut min = std::u32::MAX;
    for lay in layers {
        let mut counter = 0;
        for seq in lay.clone() {
            for num in seq.chars() {
                let num = num.to_digit(10).unwrap() as i64;
                if num == digit {
                    counter += 1;
                }
            }
        }
        if counter < min {
            min = counter;
            ret = lay;
        }
    }

    ret
}

fn count_digits(layer: Vec<String>, digit: i64) -> i64 {
    let mut counter = 0;
    for seq in layer {
        for num in seq.chars() {
            let num = num.to_digit(10).unwrap() as i64;
            if num == digit {
                counter += 1;
            }
        }
    }

    counter
}

impl Day8 {
    fn part1(&self, data: String, w: usize, h: usize) -> i64 {
        let layers = decode_layer(data, w, h);
        let lay_fewest_zero = find_layer_with_fewest(layers, 0);
        let ones = count_digits(lay_fewest_zero.clone(), 1);
        let twos = count_digits(lay_fewest_zero, 2);

        ones * twos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate() {
        let d = Day8::new("");
        assert_eq!(d.part1("123456789012".to_string(), 3, 2), 1);

        let d = Day8::new("input/day8");
        let data = std::fs::read_to_string(d.filename).unwrap();
        let r = d.part1(data, 25, 6);
        assert_eq!(r, 2460);
    }
}
