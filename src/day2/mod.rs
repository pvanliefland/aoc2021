use std::fs;

struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

pub fn part_1() -> i32 {
    let file_content =
        fs::read_to_string("src/day2/input.txt").expect("Something went wrong reading the file");

    let mut pos = Position { x: 0, y: 0, aim: 0 };
    for line in file_content.lines() {
        let (command, raw_arg) = line.split_at(
            line.find(" ")
                .expect(format!("Invalid command {}", line).as_str()),
        );
        let arg = raw_arg[1..].parse::<i32>().unwrap();
        match command {
            "forward" => pos.x += arg,
            "up" => pos.y -= arg,
            "down" => pos.y += arg,
            _ => panic!("Invalid command {}", command),
        }
    }

    pos.x * pos.y
}

pub fn part_2() -> i32 {
    let file_content =
        fs::read_to_string("src/day2/input.txt").expect("Something went wrong reading the file");

    let mut pos = Position { x: 0, y: 0, aim: 0 };
    for line in file_content.lines() {
        let (command, raw_arg) = line.split_at(
            line.find(" ")
                .expect(format!("Invalid command {}", line).as_str()),
        );
        let arg = raw_arg[1..].parse::<i32>().unwrap();
        match command {
            "forward" => {
                pos.x += arg;
                pos.y += pos.aim * arg;
            }
            "up" => pos.aim -= arg,
            "down" => pos.aim += arg,
            _ => panic!("Invalid command {}", command),
        }
    }

    pos.x * pos.y
}
