use std::fs;

/// https://adventofcode.com/2021/day/1#part1
pub fn day1_1() -> i32 {
    let file_content = fs::read_to_string("src/day1/input.txt")
        .expect("Something went wrong reading the file");

    let mut inc = 0;
    let mut prev:Option<i32> = None;
    for line in file_content.lines(){
        let curr = line.parse::<i32>().unwrap();
        if !prev.is_none() && curr > prev.unwrap() {
            inc+=1;
        }
        prev = Some(curr);
    }

    inc
}

/// https://adventofcode.com/2021/day/1#part2
pub fn day1_2() -> i32 {
    let file_content = fs::read_to_string("src/day1/input.txt")
        .expect("Something went wrong reading the file");

    let mut inc = 0;
    let mut prev:Option<i32> = None;
    let lines: Vec<i32> = file_content.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let max = lines.len() - lines.len() % 3;
    for i in 0..max {
        let curr = lines[i] + lines[i+1] + lines[i+2];
        if !prev.is_none() && curr > prev.unwrap() {
            inc+=1;
        }
        prev = Some(curr);
    }

    inc
}