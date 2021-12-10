use grid::*;

struct Heightmap {
    grid: Grid<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Height(u32, usize, usize);

impl Heightmap {
    pub fn left(&self, row: usize, col: usize) -> Option<Height> {
        (if col == 0 {
            None
        } else {
            self.grid.get(row, col - 1).copied()
        })
        .map(|n| Height(n, row, col - 1))
    }

    pub fn right(&self, row: usize, col: usize) -> Option<Height> {
        self.grid
            .get(row, col + 1)
            .copied()
            .map(|n| Height(n, row, col + 1))
    }

    pub fn down(&self, row: usize, col: usize) -> Option<Height> {
        self.grid
            .get(row + 1, col)
            .copied()
            .map(|n| Height(n, row + 1, col))
    }

    pub fn up(&self, row: usize, col: usize) -> Option<Height> {
        (if row == 0 {
            None
        } else {
            self.grid.get(row - 1, col).copied()
        })
        .map(|n| Height(n, row - 1, col))
    }

    pub fn cell(&self, row: usize, col: usize) -> Option<Height> {
        self.grid
            .get(row, col)
            .copied()
            .map(|n| Height(n, row, col))
    }

    pub fn is_valley(&self, row: usize, col: usize) -> bool {
        let cell = self.cell(row, col).unwrap();
        [
            self.left(row, col),
            self.right(row, col),
            self.down(row, col),
            self.up(row, col),
        ]
        .into_iter()
        .filter_map(|h| h)
        .all(|n| n.0 > cell.0)
    }

    pub fn basin(&self, row: usize, col: usize) -> Vec<Height> {
        let cell = self.cell(row, col).unwrap();
        let mut basin: Vec<Height> = vec![cell];
        let mut edges: Vec<Height> = basin.clone();

        loop {
            if edges.is_empty() {
                break;
            }

            let mut new_edges: Vec<Height> = Vec::new();
            for edge in edges.iter() {
                let mut neighbors: Vec<Height> = [
                    self.left(edge.1, edge.2),
                    self.right(edge.1, edge.2),
                    self.down(edge.1, edge.2),
                    self.up(edge.1, edge.2),
                ]
                .into_iter()
                .filter_map(|n| n)
                .filter(|n| n.0 != 9 && n.0 > edge.0)
                .collect();
                new_edges.append(&mut neighbors);
            }

            basin.append(&mut (new_edges.clone()));

            edges.clear();
            edges.append(&mut new_edges);
        }

        basin.sort();
        basin.dedup();
        basin
    }
}

#[allow(dead_code)]
pub fn run() {
    let heightmap = {
        let input = include_str!("../inputs/09.txt");
        let lines: Vec<&str> = input.lines().collect();
        let cols = lines[0].len();
        let input: Vec<u32> = lines
            .iter()
            .flat_map(|l| l.chars().filter_map(|n| n.to_digit(10)))
            .collect();
        let grid = Grid::from_vec(input, cols);
        Heightmap { grid }
    };

    let mut basins: Vec<Vec<u32>> = Vec::new();
    for row in 0..heightmap.grid.rows() {
        for col in 0..heightmap.grid.cols() {
            if heightmap.is_valley(row, col) {
                basins.push(heightmap.basin(row, col).iter().map(|n| n.0).collect());
            }
        }
    }

    basins.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

    let score: usize = basins
        .iter()
        .rev()
        .take(3)
        .map(|b| b.len())
        .fold(1usize, |acc, i| acc * i);
    println!("score: {}", score);
}
