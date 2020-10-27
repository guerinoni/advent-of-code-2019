/*
 * --- Day 3: Crossed Wires ---
 * The gravity assist was successful, and you're well on your way to the Venus refuelling station.
 * During the rush back on Earth, the fuel management system wasn't completely installed, so that's next on the priority list.
 *
 * Opening the front panel reveals a jumble of wires. Specifically, two wires are connected to a central port and extend outward on a grid.
 * You trace the path each wire takes as it leaves the central port, one wire per line of text (your puzzle input).
 *
 * The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, you need to find the intersection point closest to the central port.
 * Because the wires are on a grid, use the Manhattan distance for this measurement. While the wires do technically cross right at
 * the central port where they both start, this point does not count, nor does a wire count as crossing with itself.
 *
 * The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, you need to find the intersection point closest to the central port.
 * Because the wires are on a grid, use the Manhattan distance for this measurement. While the wires do technically cross right at
 * the central port where they both start, this point does not count, nor does a wire count as crossing with itself.
 *
 * For example, if the first wire's path is R8,U5,L5,D3, then starting from the central port (o), it goes right 8, up 5, left 5, and finally down 3:
 *
 * ...........
 * ...........
 * ...........
 * ....+----+.
 * ....|....|.
 * ....|....|.
 * ....|....|.
 * .........|.
 * .o-------+.
 * ...........
 * Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down 4, and left 4:
 *
 * ...........
 * .+-----+...
 * .|.....|...
 * .|..+--X-+.
 * .|..|..|.|.
 * .|.-X--+.|.
 * .|..|....|.
 * .|.......|.
 * .o-------+.
 * ...........
 * These wires cross at two locations (marked X), but the lower-left one is closer to the central port: its distance is 3 + 3 = 6.
 *
 * Here are a few more examples:
 *
 * R75,D30,R83,U83,L12,D49,R71,U7,L72
 * U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
 *
 * R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
 * U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
 * What is the Manhattan distance from the central port to the closest intersection?
 */

// R1006,U867,R355,D335,L332,U787,L938,U987,L234,U940,R393,D966,R57,U900,R619,D693,L606,U272,L45,D772,R786,U766,R860,U956,L346,D526,R536,D882,L156,U822,L247,D279,R515,U467,R208,D659,R489,D295,R18,D863,L360,D28,R674,U203,L276,U518,L936,D673,L501,D414,L635,U497,R402,D530,L589,D247,L140,U697,R626,D130,L109,D169,L316,D2,R547,D305,L480,U871,R551,D48,L710,D655,R562,D395,L925,D349,L795,U308,L861,D265,R88,U116,L719,D204,R995,D197,R167,U786,R459,U978,L506,D232,L37,U530,L808,D75,R79,D469,L851,D152,R793,D362,L293,D760,L422,U516,L400,D275,L498,U877,R202,D302,L89,U924,L55,U161,L945,D578,R861,U853,R358,D427,L776,U571,R670,D29,R52,D116,R879,U359,R493,D872,L567,U382,R804,D168,R316,D376,R711,U627,R36,D241,R876,U407,L481,D360,R679,U561,L947,U449,R232,U176,R677,U850,R165,D257,R683,D666,L31,U237,L906,U810,R198,U890,L462,D928,R915,D778,L689,U271,L486,D918,L995,U61,R748,U516,L80,D109,L328,U649,L784,D546,R584,D751,L543,U217,L976,D400,L795,U332,R453,U399,L761,U823,R142,U532,R133,U255,R722,D726,L862,D845,L813,U981,R272,D800,L918,D712,R259,U972,R323,D214,R694,D629,L817,D814,L741,U111,L678,D627,L743,D509,R195,U875,R46,D344,L361,D102,L656,U897,L841,U124,L95,D770,L785,U767,L504,D309,L955,D142,L401,U914,R117,D897,R715,D117,R675,U248,R182,D725,L751,U562,R385,D120,R730,U185,L842,D446,L432,U640,R994,D482,R576,U915,R645,U109,R77,D983,L327,D209,R686,D486,R566,D406,R130,D759,R441,U895,R597,U443,L773,D704,R219,U222,R244,D11,L723,U804,L264,D121,L81,D454,R279,D642,L773,D653,R127,D199,R119,U509,L530
// L1003,D933,L419,D202,L972,U621,L211,U729,R799,U680,R925,U991,L167,U800,R198,U214,R386,D385,R117,D354,L914,D992,L519,U797,L28,D125,R163,D894,R390,D421,L75,D577,L596,U95,L403,U524,L160,D39,R209,D373,L464,U622,L824,D750,L857,U658,L109,D188,R357,D295,L227,U904,L268,U814,L483,U897,R785,U194,R865,U300,L787,U812,L321,D637,R761,U560,R800,U281,R472,D283,L490,D629,L207,D589,L310,D980,R613,U129,R668,U261,R82,D594,R627,D210,L865,U184,R387,U995,R497,U68,L776,U657,R559,D38,R981,D485,L196,D934,R313,D128,R269,D225,L32,U677,R425,U728,L665,D997,R271,D847,R715,U300,L896,D481,L30,U310,L793,D600,L219,D944,R197,D945,L564,D603,L225,U413,L900,U876,R281,D26,R449,D506,L846,D979,L817,D794,R309,D841,R735,U11,R373,U530,R74,D534,L668,U185,L972,D436,L377,D164,L88,U249,L8,D427,R711,D530,L850,D921,L644,U804,L388,U500,L813,D223,L572,U246,R309,U241,R185,D48,L545,U678,L344,D964,L772,D985,L178,U686,R937,U821,R214,D346,L648,D824,L943,D651,R98,D225,R832,D883,L814,D894,L995,D975,R440,D502,L177,D320,R675,U5,R188,D866,R974,U169,R432,D627,L424,D5,L273,U184,R657,U830,R681,U610,R170,U106,L726,D861,L257,D381,L918,D607,L820,D757,R556,D621,R21,U510,L575,D545,L590,D302,R446,D225,L164,D817,L520,D204,L33,U272,L648,D478,R945,U369,L924,D932,R46,D584,R630,U592,R613,U136,R253,D343,L983,U328,L442,D311,L258,U173,L574,U658,R283,D927,L247,D37,R36,D61,L692,U663,L207,U48,L114,U511,L924,U229,L221,D504,R345,U51,R464,D516,L115,D311,L71,D418,R378,D173,R154,U436,L403,D871,L765,D803,R630,U108,L79,U625,R77,U176,R911

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn manhattan(a: &Point, b: &Point) -> u64 {
    (a.x - b.x).abs() as u64 + (a.y - b.y).abs() as u64
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn length(&self) -> u64 {
        if self.from.x == self.to.x {
            let max = std::cmp::max(self.to.y, self.from.y);
            let min = std::cmp::min(self.to.y, self.from.y);
            return (max - min).abs() as u64;
        }

        let max = std::cmp::max(self.to.x, self.from.x);
        let min = std::cmp::min(self.to.x, self.from.x);
        return (max - min).abs() as u64;
    }

    fn instersection_on_x(&self, p: &Point) -> bool {
        p.x <= self.to.x && p.x >= self.from.x
    }

    fn instersection_on_y(&self, p: &Point) -> bool {
        p.y <= self.to.y && p.y >= self.from.y
    }
}

fn get_points(vector: &Vec<&str>) -> Vec<Point> {
    let origin = Point { x: 0, y: 0 };
    let mut points = vec![origin];

    for str in vector {
        let dir = str.chars().nth(0).unwrap();
        let num = str[1..].trim().parse::<i32>().unwrap();
        let old_p = points.last().unwrap();
        let mut new_p = Point {
            x: old_p.x,
            y: old_p.y,
        };

        match dir {
            'R' => new_p.x += num,
            'L' => new_p.x -= num,
            'D' => new_p.y -= num,
            'U' => new_p.y += num,
            _ => panic!("unexpected dir"),
        }
        points.push(new_p);
    }

    points
}

fn make_lines(points: &Vec<Point>) -> Vec<Line> {
    let mut j: usize = 2;
    let mut lines = vec![Line {
        from: points[0],
        to: points[1],
    }];

    for i in 1..(points.len() - 1) {
        let line = Line {
            from: points[i],
            to: points[j],
        };
        lines.push(line);
        j += 1;
    }

    lines
}

fn get_instersection(v: &Line, vv: &Line) -> Option<Point> {
    let p0_x = v.from.x as f64;
    let p0_y = v.from.y as f64;

    let p1_x = v.to.x as f64;
    let p1_y = v.to.y as f64;

    let p2_x = vv.from.x as f64;
    let p2_y = vv.from.y as f64;

    let p3_x = vv.to.x as f64;
    let p3_y = vv.to.y as f64;

    let s1_x = p1_x - p0_x;
    let s1_y = p1_y - p0_y;
    let s2_x = p3_x - p2_x;
    let s2_y = p3_y - p2_y;

    let det = -s2_x * s1_y + s1_x * s2_y;
    let s = (-s1_y * (p0_x - p2_x) + s1_x * (p0_y - p2_y)) / det;
    let t = (s2_x * (p0_y - p2_y) - s2_y * (p0_x - p2_x)) / det;

    if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
        let ix = (p0_x + (t * s1_x)) as i32;
        let iy = (p0_y + (t * s1_y)) as i32;
        return Some(Point { x: ix, y: iy });
    }

    None
}

fn find_min_intersection_distance(v1: &Vec<Line>, v2: &Vec<Line>) -> u64 {
    let mut intersections = vec![];
    for v in v1 {
        for vv in v2 {
            if let Some(point) = get_instersection(v, vv) {
                intersections.push(point);
            }
        }
    }

    let origin = Point { x: 0, y: 0 };
    intersections.retain(|p| p != &origin);

    let min = intersections
        .iter()
        .min_by(|inter1, inter2| manhattan(&origin, inter1).cmp(&manhattan(&origin, inter2)))
        .unwrap();

    manhattan(&origin, &min)
}

fn part1(wire1: &Vec<&str>, wire2: &Vec<&str>) -> u64 {
    let points1 = get_points(wire1);
    let points2 = get_points(wire2);
    let lines1 = make_lines(&points1);
    let lines2 = make_lines(&points2);

    find_min_intersection_distance(&lines1, &lines2)
}

/*
 * --- Part Two ---
 * It turns out that this circuit is very timing-sensitive; you actually need to minimize the signal delay.
 *
 * To do this, calculate the number of steps each wire takes to reach each intersection;
 * choose the intersection where the sum of both wires' steps is lowest. If a wire visits a position on the grid multiple times,
 * use the steps value from the first time it visits that position when calculating the total value of a specific intersection.
 *
 * The number of steps a wire takes is the total number of grid squares the wire has entered to get to that location,
 * including the intersection being considered. Again consider the example from above:
 *
 * ...........
 * .+-----+...
 * .|.....|...
 * .|..+--X-+.
 * .|..|..|.|.
 * .|.-X--+.|.
 * .|..|....|.
 * .|.......|.
 * .o-------+.
 * ...........
 * In the above example, the intersection closest to the central port is reached after 8+5+5+2 = 20 steps by the first wire
 * and 7+6+4+3 = 20 steps by the second wire for a total of 20+20 = 40 steps.
 *
 * However, the top-right intersection is better: the first wire takes only 8+5+2 = 15 and the second wire takes only 7+6+2 = 15,
 * a total of 15+15 = 30 steps.
 *
 * Here are the best steps for the extra examples from above:
 *
 * R75,D30,R83,U83,L12,D49,R71,U7,L72
 * U62,R66,U55,R34,D71,R55,D58,R83 = 610 steps
 * R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
 * U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = 410 steps
 * What is the fewest combined steps the wires must take to reach an intersection?
 */

fn distance(p1: &Point, p2: &Point) -> f64 {
    let v = ((p2.x - p1.x) * (p2.x - p1.x) + (p2.y - p1.y) * (p2.y - p1.y)) as f64;
    v.sqrt()
}

fn find_min_steps(w1: &Vec<Line>, w2: &Vec<Line>) -> u64 {
    let origin = Point { x: 0, y: 0 };

    let mut min_steps = std::u64::MAX;
    let mut steps1 = 0;

    for v in w1 {
        steps1 += v.length();

        let mut steps2 = 0;
        for vv in w2 {
            steps2 += vv.length();

            if let Some(point) = get_instersection(v, vv) {
                if point == origin {
                    continue;
                }

                let s1 = steps1 - (distance(&point, &v.to) as u64);
                let s2 = steps2 - (distance(&point, &vv.to) as u64);
                min_steps = std::cmp::min(min_steps, s1 + s2);
            }
        }
    }

    min_steps
}

fn part2(wire1: &Vec<&str>, wire2: &Vec<&str>) -> u64 {
    let points1 = get_points(wire1);
    let points2 = get_points(wire2);
    let lines1 = make_lines(&points1);
    let lines2 = make_lines(&points2);

    find_min_steps(&lines1, &lines2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1_simple() {
        let input1 = "R8,U5,L5,D3".split(',');
        let wire1 = input1.collect::<Vec<_>>();
        let input2 = "U7,R6,D4,L4".split(',');
        let wire2 = input2.collect::<Vec<_>>();
        assert_eq!(part1(&wire1, &wire2), 6);
    }

    #[test]
    fn day3_part1_medium() {
        let input1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72".split(',');
        let wire1 = input1.collect::<Vec<_>>();
        let input2 = "U62,R66,U55,R34,D71,R55,D58,R83".split(',');
        let wire2 = input2.collect::<Vec<_>>();
        assert_eq!(part1(&wire1, &wire2), 159);
    }

    #[test]
    fn day3_part1_hard() {
        let input1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".split(',');
        let wire1 = input1.collect::<Vec<_>>();
        let input2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".split(',');
        let wire2 = input2.collect::<Vec<_>>();
        assert_eq!(part1(&wire1, &wire2), 135);
    }

    #[test]
    fn day3_part1() {
        let input1 = "R1006,U867,R355,D335,L332,U787,L938,U987,L234,U940,R393,D966,R57,U900,R619,D693,L606,U272,L45,D772,R786,U766,R860,U956,L346,D526,R536,D882,L156,U822,L247,D279,R515,U467,R208,D659,R489,D295,R18,D863,L360,D28,R674,U203,L276,U518,L936,D673,L501,D414,L635,U497,R402,D530,L589,D247,L140,U697,R626,D130,L109,D169,L316,D2,R547,D305,L480,U871,R551,D48,L710,D655,R562,D395,L925,D349,L795,U308,L861,D265,R88,U116,L719,D204,R995,D197,R167,U786,R459,U978,L506,D232,L37,U530,L808,D75,R79,D469,L851,D152,R793,D362,L293,D760,L422,U516,L400,D275,L498,U877,R202,D302,L89,U924,L55,U161,L945,D578,R861,U853,R358,D427,L776,U571,R670,D29,R52,D116,R879,U359,R493,D872,L567,U382,R804,D168,R316,D376,R711,U627,R36,D241,R876,U407,L481,D360,R679,U561,L947,U449,R232,U176,R677,U850,R165,D257,R683,D666,L31,U237,L906,U810,R198,U890,L462,D928,R915,D778,L689,U271,L486,D918,L995,U61,R748,U516,L80,D109,L328,U649,L784,D546,R584,D751,L543,U217,L976,D400,L795,U332,R453,U399,L761,U823,R142,U532,R133,U255,R722,D726,L862,D845,L813,U981,R272,D800,L918,D712,R259,U972,R323,D214,R694,D629,L817,D814,L741,U111,L678,D627,L743,D509,R195,U875,R46,D344,L361,D102,L656,U897,L841,U124,L95,D770,L785,U767,L504,D309,L955,D142,L401,U914,R117,D897,R715,D117,R675,U248,R182,D725,L751,U562,R385,D120,R730,U185,L842,D446,L432,U640,R994,D482,R576,U915,R645,U109,R77,D983,L327,D209,R686,D486,R566,D406,R130,D759,R441,U895,R597,U443,L773,D704,R219,U222,R244,D11,L723,U804,L264,D121,L81,D454,R279,D642,L773,D653,R127,D199,R119,U509,L530".split(',');
        let wire1 = input1.collect::<Vec<_>>();
        let input2 ="L1003,D933,L419,D202,L972,U621,L211,U729,R799,U680,R925,U991,L167,U800,R198,U214,R386,D385,R117,D354,L914,D992,L519,U797,L28,D125,R163,D894,R390,D421,L75,D577,L596,U95,L403,U524,L160,D39,R209,D373,L464,U622,L824,D750,L857,U658,L109,D188,R357,D295,L227,U904,L268,U814,L483,U897,R785,U194,R865,U300,L787,U812,L321,D637,R761,U560,R800,U281,R472,D283,L490,D629,L207,D589,L310,D980,R613,U129,R668,U261,R82,D594,R627,D210,L865,U184,R387,U995,R497,U68,L776,U657,R559,D38,R981,D485,L196,D934,R313,D128,R269,D225,L32,U677,R425,U728,L665,D997,R271,D847,R715,U300,L896,D481,L30,U310,L793,D600,L219,D944,R197,D945,L564,D603,L225,U413,L900,U876,R281,D26,R449,D506,L846,D979,L817,D794,R309,D841,R735,U11,R373,U530,R74,D534,L668,U185,L972,D436,L377,D164,L88,U249,L8,D427,R711,D530,L850,D921,L644,U804,L388,U500,L813,D223,L572,U246,R309,U241,R185,D48,L545,U678,L344,D964,L772,D985,L178,U686,R937,U821,R214,D346,L648,D824,L943,D651,R98,D225,R832,D883,L814,D894,L995,D975,R440,D502,L177,D320,R675,U5,R188,D866,R974,U169,R432,D627,L424,D5,L273,U184,R657,U830,R681,U610,R170,U106,L726,D861,L257,D381,L918,D607,L820,D757,R556,D621,R21,U510,L575,D545,L590,D302,R446,D225,L164,D817,L520,D204,L33,U272,L648,D478,R945,U369,L924,D932,R46,D584,R630,U592,R613,U136,R253,D343,L983,U328,L442,D311,L258,U173,L574,U658,R283,D927,L247,D37,R36,D61,L692,U663,L207,U48,L114,U511,L924,U229,L221,D504,R345,U51,R464,D516,L115,D311,L71,D418,R378,D173,R154,U436,L403,D871,L765,D803,R630,U108,L79,U625,R77,U176,R911".split(',');
        let wire2 = input2.collect::<Vec<_>>();
        assert_eq!(part1(&wire1, &wire2), 1431);
    }

    #[test]
    fn day3_part2_simple() {
        let input1 = "R8,U5,L5,D3".split(',');
        let wire1 = input1.collect::<Vec<_>>();
        let input2 = "U7,R6,D4,L4".split(',');
        let wire2 = input2.collect::<Vec<_>>();
        assert_eq!(part2(&wire1, &wire2), 30);
    }

    #[test]
    fn day3_part2_medium() {
        let input1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72".split(',');
        let wire1 = input1.collect::<Vec<_>>();
        let input2 = "U62,R66,U55,R34,D71,R55,D58,R83".split(',');
        let wire2 = input2.collect::<Vec<_>>();
        assert_eq!(part2(&wire1, &wire2), 610);
    }

    #[test]
    fn day3_part2_hard() {
        let input1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".split(',');
        let wire1 = input1.collect::<Vec<_>>();
        let input2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".split(',');
        let wire2 = input2.collect::<Vec<_>>();
        assert_eq!(part2(&wire1, &wire2), 410);
    }

    #[test]
    fn day3_part2() {
        let input1 = "R1006,U867,R355,D335,L332,U787,L938,U987,L234,U940,R393,D966,R57,U900,R619,D693,L606,U272,L45,D772,R786,U766,R860,U956,L346,D526,R536,D882,L156,U822,L247,D279,R515,U467,R208,D659,R489,D295,R18,D863,L360,D28,R674,U203,L276,U518,L936,D673,L501,D414,L635,U497,R402,D530,L589,D247,L140,U697,R626,D130,L109,D169,L316,D2,R547,D305,L480,U871,R551,D48,L710,D655,R562,D395,L925,D349,L795,U308,L861,D265,R88,U116,L719,D204,R995,D197,R167,U786,R459,U978,L506,D232,L37,U530,L808,D75,R79,D469,L851,D152,R793,D362,L293,D760,L422,U516,L400,D275,L498,U877,R202,D302,L89,U924,L55,U161,L945,D578,R861,U853,R358,D427,L776,U571,R670,D29,R52,D116,R879,U359,R493,D872,L567,U382,R804,D168,R316,D376,R711,U627,R36,D241,R876,U407,L481,D360,R679,U561,L947,U449,R232,U176,R677,U850,R165,D257,R683,D666,L31,U237,L906,U810,R198,U890,L462,D928,R915,D778,L689,U271,L486,D918,L995,U61,R748,U516,L80,D109,L328,U649,L784,D546,R584,D751,L543,U217,L976,D400,L795,U332,R453,U399,L761,U823,R142,U532,R133,U255,R722,D726,L862,D845,L813,U981,R272,D800,L918,D712,R259,U972,R323,D214,R694,D629,L817,D814,L741,U111,L678,D627,L743,D509,R195,U875,R46,D344,L361,D102,L656,U897,L841,U124,L95,D770,L785,U767,L504,D309,L955,D142,L401,U914,R117,D897,R715,D117,R675,U248,R182,D725,L751,U562,R385,D120,R730,U185,L842,D446,L432,U640,R994,D482,R576,U915,R645,U109,R77,D983,L327,D209,R686,D486,R566,D406,R130,D759,R441,U895,R597,U443,L773,D704,R219,U222,R244,D11,L723,U804,L264,D121,L81,D454,R279,D642,L773,D653,R127,D199,R119,U509,L530".split(",");
        let wire1 = input1.collect::<Vec<_>>();
        let input2 = "L1003,D933,L419,D202,L972,U621,L211,U729,R799,U680,R925,U991,L167,U800,R198,U214,R386,D385,R117,D354,L914,D992,L519,U797,L28,D125,R163,D894,R390,D421,L75,D577,L596,U95,L403,U524,L160,D39,R209,D373,L464,U622,L824,D750,L857,U658,L109,D188,R357,D295,L227,U904,L268,U814,L483,U897,R785,U194,R865,U300,L787,U812,L321,D637,R761,U560,R800,U281,R472,D283,L490,D629,L207,D589,L310,D980,R613,U129,R668,U261,R82,D594,R627,D210,L865,U184,R387,U995,R497,U68,L776,U657,R559,D38,R981,D485,L196,D934,R313,D128,R269,D225,L32,U677,R425,U728,L665,D997,R271,D847,R715,U300,L896,D481,L30,U310,L793,D600,L219,D944,R197,D945,L564,D603,L225,U413,L900,U876,R281,D26,R449,D506,L846,D979,L817,D794,R309,D841,R735,U11,R373,U530,R74,D534,L668,U185,L972,D436,L377,D164,L88,U249,L8,D427,R711,D530,L850,D921,L644,U804,L388,U500,L813,D223,L572,U246,R309,U241,R185,D48,L545,U678,L344,D964,L772,D985,L178,U686,R937,U821,R214,D346,L648,D824,L943,D651,R98,D225,R832,D883,L814,D894,L995,D975,R440,D502,L177,D320,R675,U5,R188,D866,R974,U169,R432,D627,L424,D5,L273,U184,R657,U830,R681,U610,R170,U106,L726,D861,L257,D381,L918,D607,L820,D757,R556,D621,R21,U510,L575,D545,L590,D302,R446,D225,L164,D817,L520,D204,L33,U272,L648,D478,R945,U369,L924,D932,R46,D584,R630,U592,R613,U136,R253,D343,L983,U328,L442,D311,L258,U173,L574,U658,R283,D927,L247,D37,R36,D61,L692,U663,L207,U48,L114,U511,L924,U229,L221,D504,R345,U51,R464,D516,L115,D311,L71,D418,R378,D173,R154,U436,L403,D871,L765,D803,R630,U108,L79,U625,R77,U176,R911".split(",");
        let wire2 = input2.collect::<Vec<_>>();
        assert_eq!(part2(&wire1, &wire2), 48012);
    }
}
