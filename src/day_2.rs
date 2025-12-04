use crate::utils;
use itertools;
use itertools::Itertools;
use std::ops::RangeInclusive;
use utf8_chars::BufReadCharsExt;

pub fn task_1(filename: &str) -> usize {
    process(filename, is_invalid_task_1)
}

pub fn task_2(filename: &str) -> usize {
    process(filename, is_invalid_task_2)
}

fn process(filename: & str, test: fn(&usize) -> bool) -> usize {
    utils::get_reader(filename)
        .expect("Unable to open file for reading")
        .chars()
        .map_while(Result::ok)
        .batching(|it| Some(it.take_while(|x| ','.ne(x)).join("")))
        .map_while(|x| get_range(x.as_str()))
        .flatten()
        .filter(|x| test(x))
        .sum()
}

fn get_range(s: &str) -> Option<RangeInclusive<usize>> {
    if let Some((a, b)) = s.split_once('-') {
        Some(a.parse::<usize>().unwrap()..=b.parse::<usize>().unwrap())
    } else {
        None
    }
}

fn is_invalid_task_1(id: &usize) -> bool {
    let s = id.to_string();
    let l= s.len();
    l%2==0 && s[..l/2] == s[l/2..]
}

fn is_invalid_task_2(id: &usize) -> bool {
    let s: String = id.to_string();
    s.repeat(2)[1..(2*s.len()-1)].contains(&s)
}