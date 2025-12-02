use std::ops::{Div};
use crate::utils;

pub fn day_1_task_1(start: i32, filename: &str) -> i32 {
    utils::read_lines(filename)
        .expect("Unable to open file for reading")
        .map_while(Result::ok)
        .map(parse_movement)
        .fold((0, start), |(a, b), i| {
            let c = (b == 0) as i32;
            let n = (b + i).rem_euclid(100);
            (a + c , n)
        }).0
}

pub fn day_1_task_2(start: i32, filename: &str) -> i32 {
    utils::read_lines(filename)
        .expect("Unable to open file for reading")
        .map_while(Result::ok)
        .map(parse_movement)
        .fold((0, start), |(a, b), i| {
            let c1 = (b + i).abs().div(100); // number of whole turns past 0
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
