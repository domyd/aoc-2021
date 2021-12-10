#[allow(dead_code)]
pub fn run() {
    let input = include_str!("../inputs/06.txt");
    let lines: Vec<&str> = input.lines().collect();
    let input: Vec<usize> = lines[0]
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    let mut fish: [usize; 9] = [0; 9];

    for &i in input.iter() {
        fish[i] += 1;
    }

    for day in 1..=256 {
        let ready_fish = fish[0];
        fish[0] = 0;
        fish.rotate_left(1);
        fish[6] += ready_fish;
        fish[8] += ready_fish;
        let count: usize = fish.iter().sum();
        println!("after {} days: {} fish", day, count);
    }
}
