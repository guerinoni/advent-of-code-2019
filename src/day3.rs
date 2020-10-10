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

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    from: Point,
    to: Point,
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

    points[1..].to_vec()
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

fn split_lines(lines: Vec<Line>) -> (Vec<Line>, Vec<Line>) {
    let verticals = lines
        .clone()
        .into_iter()
        .filter(|l| l.from.x == l.to.x)
        .collect::<Vec<Line>>();

    let horizontals = lines
        .into_iter()
        .filter(|l| l.from.y == l.to.y)
        .collect::<Vec<Line>>();

    (verticals, horizontals)
}

fn is_there_instersection(v: &Line, h: &Line) -> bool {
    let min_h = std::cmp::min(h.from.x, h.to.x);
    let max_h = std::cmp::max(h.from.x, h.to.x);
    let min_v = std::cmp::min(v.from.y, v.to.y);
    let max_v = std::cmp::max(v.from.y, v.to.y);
    (v.from.x <= max_h && v.from.x >= min_h) && (h.from.y <= max_v && h.from.y >= min_v)
}

fn find_intersection(vv: &Vec<Line>, hh: &Vec<Line>) -> i32 {
    let mut intersection = Point { x: 10000, y: 10000 };
    for v in vv {
        for h in hh {
            if is_there_instersection(v, h) {
                let tmp = Point {
                    x: v.from.x,
                    y: h.from.y,
                };

                if intersection.x.abs() + intersection.y.abs() > tmp.x.abs() + tmp.y.abs() {
                    intersection = tmp;
                }
            }
        }
    }

    intersection.x.abs() + intersection.y.abs()
}

fn part1(wire1: &Vec<&str>, wire2: &Vec<&str>) -> i32 {
    let points1 = get_points(wire1);
    let points2 = get_points(wire2);
    let line1 = make_lines(&points1);
    let line2 = make_lines(&points2);
    let (v1, h1) = split_lines(line1);
    let (v2, h2) = split_lines(line2);
    let int1 = find_intersection(&v1, &h2);
    let int2 = find_intersection(&v2, &h1);

    std::cmp::min(int1, int2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_simple() {
        let input1 = "R8,U5,L5,D3".split(',');
        let wire1 = input1.collect::<Vec<_>>();
        let input2 = "U7,R6,D4,L4".split(',');
        let wire2 = input2.collect::<Vec<_>>();
        assert_eq!(part1(&wire1, &wire2), 6);
    }

    #[test]
    fn day3_medium() {
        let input1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72".split(',');
        let wire1 = input1.collect::<Vec<_>>();
        let input2 = "U62,R66,U55,R34,D71,R55,D58,R83".split(',');
        let wire2 = input2.collect::<Vec<_>>();
        assert_eq!(part1(&wire1, &wire2), 159);
    }

    #[test]
    fn day3_hard() {
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
}
