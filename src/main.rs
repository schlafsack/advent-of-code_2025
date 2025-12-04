mod day_1;
mod utils;
mod day_2;

fn main() {
    //day 1
    println!("day 1 task 2: {}", day_1::task_1("./input/1.test.txt"));
    println!("day 1 task 2: {}", day_1::task_2("./input/1.test.txt"));
    //day 2
    println!("day 2 task 1: {}", day_2::task_1("./input/2.txt"));
    println!("day 2 task 2: {}", day_2::task_2("./input/2.txt"));
}
