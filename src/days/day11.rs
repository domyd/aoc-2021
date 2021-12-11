use std::collections::{HashSet, VecDeque};

use crate::utils::vec_into_hashmap;

#[allow(dead_code)]
pub fn run() {
    let input = include_str!("../inputs/11.txt");
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|n| n.to_digit(10).map(|n| n))
                .collect()
        })
        .collect();
    let (mut map, rows, cols) = vec_into_hashmap(input);

    let dp: [(isize, isize); 8] = [
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
    ];

    for step in 1.. {
        map.iter_mut().for_each(|octo| *octo.1 += 1);

        let mut flashed = HashSet::new();
        for p in map.keys().copied().collect::<Vec<_>>() {
            let mut queue = VecDeque::new();
            if let Some(octo) = map.get(&p) {
                if *octo > 9 {
                    queue.push_back(p);
                }
            }

            while let Some(pos @ (y, x)) = queue.pop_back() {
                if flashed.contains(&pos) {
                    continue;
                }
                flashed.insert(pos);

                for dp in dp {
                    let pos = (y + dp.0, x + dp.1);
                    if let Some(octo) = map.get_mut(&pos) {
                        *octo += 1;
                        if *octo > 9 {
                            queue.push_back(pos);
                        }
                    }
                }
            }
        }

        for pos in flashed.iter() {
            *(map.get_mut(&pos).unwrap()) = 0;
        }

        if flashed.len() == rows * cols {
            eprintln!("flashed {} octopuses in step {}", flashed.len(), step);
            break;
        }
    }
}
