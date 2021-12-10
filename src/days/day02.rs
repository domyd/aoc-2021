#[derive(Debug, Clone, Copy)]
struct Position {
    pub depth: i32,
    pub position: i32,
    pub aim: i32,
}

impl Position {
    pub fn new() -> Position {
        Position {
            depth: 0,
            position: 0,
            aim: 0,
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    let input = include_str!("../inputs/02.txt");
    let mut pos = Position::new();
    for l in input.lines() {
        let command = l.split(' ').collect::<Vec<&str>>();
        let (cmd, mag) = (command[0], command[1].parse::<i32>().unwrap());
        match (cmd, mag) {
            ("forward", x) => {
                pos.position += x;
                pos.depth += pos.aim * x;
            }
            ("down", x) => pos.aim += x,
            ("up", x) => pos.aim -= x,
            _ => {}
        };
    }
    dbg!(pos);
    println!("position * depth: {}", pos.depth * pos.position);
}
