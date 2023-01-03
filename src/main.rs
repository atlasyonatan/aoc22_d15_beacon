use regex::Regex;
use std::ops::RangeInclusive;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let y = 2000000;
    // let y = 10;
    let path = Path::new("../input.txt");
    let file = File::open(path).unwrap();
    let reg =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let inputs: Vec<([i32; 2], [i32; 2])> = io::BufReader::new(file)
        .lines()
        .map(|result| result.unwrap())
        .map(|string| {
            let captures = reg.captures(&string).unwrap();
            let mut iter = (1..=4)
                .map(|n| captures.get(n).unwrap())
                .map(|m| m.as_str().parse::<i32>().unwrap());
            let sensor = [iter.next().unwrap(), iter.next().unwrap()];
            let beacon = [iter.next().unwrap(), iter.next().unwrap()];
            (sensor, beacon)
        })
        .collect();

    let ranges: Vec<RangeInclusive<i32>> = inputs
        .iter()
        .filter_map(|(sensor, beacon)| {
            let contour_function = manhatten_contour(sensor, manhatten_distance(sensor, beacon));
            match contour_function(y) {
                Some(contour) => Some(contour[0]..=contour[1]),
                None => None,
            }
        })
        .collect();

    let min = ranges.iter().map(|range| range.start()).min().unwrap();
    let max = ranges.iter().map(|range| range.end()).max().unwrap();
    let offset = min;
    let size = max - min + 1;
    // println!("offset: {}", offset);
    // println!("size: {}", size);
    let mut no_beacons = vec![false; size as usize];
    for range in ranges.iter() {
        for i in range.clone() {
            no_beacons[(i - offset) as usize] = true;
        }
    }
    for (_, beacon) in inputs.iter() {
        if beacon[1] == y {
            no_beacons[(beacon[0] - offset) as usize] = false;
        }
    }
    println!("part 1: {}", no_beacons.iter().filter(|&&b| b).count())
}

fn manhatten_distance(p1: &[i32; 2], p2: &[i32; 2]) -> i32 {
    (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs()
}

fn manhatten_contour<'a>(
    center: &'a [i32; 2],
    distance: i32,
) -> impl Fn(i32) -> Option<[i32; 2]> + 'a {
    move |y| {
        let distance = distance - (center[1] - y).abs();
        (distance >= 0).then(|| [center[0] - distance, center[0] + distance])
    }
}
