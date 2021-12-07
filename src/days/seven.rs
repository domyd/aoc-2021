#[allow(dead_code)]
pub fn seven() {
    let input = include_str!("../inputs/07.txt");
    let numbers: Vec<i32> = input.split(',').filter_map(|n| n.parse().ok()).collect();

    let (mut min_fuel, mut min_pos) = (i32::MAX, 0);
    for p in 0..=(numbers.iter().cloned().fold(0, i32::max)) {
        let mut fuel = 0;
        for crab in numbers.iter() {
            let movement = {
                let n = i32::abs(crab - p);
                (n * (n + 1)) / 2
            };
            fuel += movement;
        }

        if fuel < min_fuel {
            min_fuel = fuel;
            min_pos = p;
        }
    }

    println!("minimum fuel needed is {} @ pos {}", min_fuel, min_pos);
}
