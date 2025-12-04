use std::io::BufRead;
use crate::utils;

pub fn task_1(filename: &str) -> i32 {
    utils::get_reader(filename)
        .expect("Unable to open file for reading")
        .lines()
        .map_while(Result::ok)
        .map(parse_movement)
        .fold((0, 50), |(a, b), i| {
            let c = (b == 0) as i32;
            let n = (b + i).rem_euclid(100);
            (a + c , n)
        }).0
}

pub fn task_2(filename: &str) -> i32 {
    utils::get_reader(filename)
        .expect("Unable to open file for reading")
        .lines()
        .map_while(Result::ok)
        .map(parse_movement)
        .fold((0, 50), |(a, b), i| {
            let c1 = (b + i).abs().div_euclid(100); // number of whole turns past 0
            let c2 = (b != 0 && b + i <= 0) as i32; // number of partial turns past 0
            let n = (b + i).rem_euclid(100);
            (a + c1 + c2 , n)
        }).0
}

fn parse_movement(movement: String) -> i32 {
    movement
        .chars()
        .map(|c| match c {
            'L' => '-',
            'R' => ' ',
            _ => c,
        })
        .collect::<String>()
        .trim()
        .parse()
        .expect("Unable to parse movement.")
}
