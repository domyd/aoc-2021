use grid::*;
use std::cmp::{max, min};

#[derive(Debug, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

pub fn five() {
    let input = include_str!("../inputs/five.txt");
    let lines: Vec<&str> = input.lines().collect();
    let input: Vec<Vec<Position>> = lines
        .iter()
        .map(|l| {
            let v: Vec<Position> = l
                .split("->")
                .map(|x| {
                    x.trim()
                        .split_once(',')
                        .map(|(l, r)| Position {
                            x: l.parse::<isize>().unwrap(),
                            y: r.parse::<isize>().unwrap(),
                        })
                        .unwrap()
                })
                .collect();
            (v[0], v[1])
        })
        .map(|p| line(p.0, p.1))
        .collect();

    let mut grid: Grid<usize> = Grid::new(1000, 1000);
    for line in input {
        draw_line(&mut grid, line);
    }

    let overlap_count = grid.iter().filter(|x| **x >= 2).count();
    println!("number of overlaps: {}", overlap_count);
}

fn line(from: Position, to: Position) -> Vec<Position> {
    let (x1, x2) = (min(from.x, to.x), max(from.x, to.x));
    let (y1, y2) = (min(from.y, to.y), max(from.y, to.y));
    if from.x == to.x {
        (y1..=y2)
            .into_iter()
            .map(|i| Position { x: from.x, y: i })
            .collect()
    } else if from.y == to.y {
        (x1..=x2)
            .into_iter()
            .map(|i| Position { x: i, y: from.y })
            .collect()
    } else {
        let (diff_x, diff_y) = (x2 - x1, y2 - y1);
        if diff_x == diff_y {
            // we can make a diagonal line
            let dir: (isize, isize) = if to.x > from.x {
                if to.y > from.y {
                    (1, 1)
                } else {
                    (1, -1)
                }
            } else {
                if to.y > from.y {
                    (-1, 1)
                } else {
                    (-1, -1)
                }
            };

            (0..=diff_x)
                .into_iter()
                .map(|i| Position {
                    x: from.x + (dir.0 * i),
                    y: from.y + (dir.1 * i),
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

fn draw_line(grid: &mut Grid<usize>, line: Vec<Position>) {
    for l in line {
        let p = grid.get_mut(l.x as usize, l.y as usize).unwrap();
        (*p) += 1;
    }
}
