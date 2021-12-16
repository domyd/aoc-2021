use std::collections::VecDeque;

use crate::utils::{print_hashmap_grid, vec_into_hashmap};

#[allow(dead_code)]
pub fn run() {
    let input = include_str!("../inputs/15.txt");
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|n| n.to_digit(10).map(|n| n))
                .collect()
        })
        .collect();
    let (mut map, rows, cols) = vec_into_hashmap(input);

    dbg!((rows, cols));
    print_hashmap_grid(&map);

    let dp: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

    let mut paths = Vec::new();
    let mut q = VecDeque::new();
    q.push_back(vec![(0isize, 0isize, *map.get(&(0, 0)).unwrap())]);
    while let Some(path) = q.pop_back() {
        let elem = path.last().unwrap();
        let (y, x) = (elem.0, elem.1);
        if y == (rows - 1) as isize && x == (cols - 1) as isize {
            // eprintln!("completing path: {:?}", &path);
            paths.push(path);
            continue;
        }

        for dp in dp {
            let pos = (y + dp.0, x + dp.1);
            if let Some(neighbor) = map.get(&pos) {
                let new_elem = (pos.0, pos.1, *neighbor);
                if path.contains(&new_elem) {
                    continue;
                }
                let mut path = path.clone();
                path.push(new_elem);
                q.push_back(path);
            }
        }
    }

    dbg!(&paths.len());
}

// pub fn a_star()
