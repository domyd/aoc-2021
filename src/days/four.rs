use grid::*;
use std::fmt::Debug;

#[derive(Clone, Copy)]
struct Mark {
    pub number: u8,
    pub marked: bool,
}

impl Debug for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", &self.number))?;
        if self.marked {
            f.write_str("X")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Board {
    grid: Grid<Mark>,
    won: Option<u8>,
}

impl Board {
    pub fn new(grid: Grid<Mark>) -> Self {
        Self { grid, won: None }
    }

    pub fn mark(&mut self, n: u8) {
        if let Some(mark) = self.grid.iter_mut().find(|m| m.number == n) {
            mark.marked = true;
            self.check_completed(n);
        }
    }

    pub fn sum_unmarked(&self) -> usize {
        self.grid
            .iter()
            .filter_map(|f| {
                if f.marked {
                    None
                } else {
                    Some(f.number as usize)
                }
            })
            .sum()
    }

    pub fn score(&self) -> Option<(usize, u8)> {
        self.won.map(|d| (self.sum_unmarked() * d as usize, d))
    }

    fn check_completed(&mut self, drawing: u8) {
        let g = &self.grid;
        for i in 0..g.rows() {
            if g.iter_row(i).all(|f| f.marked) {
                self.won = Some(drawing);
            }
        }
        for i in 0..g.cols() {
            if g.iter_col(i).all(|f| f.marked) {
                self.won = Some(drawing);
            }
        }
    }
}

pub fn four() {
    let input = include_str!("../inputs/four.txt");
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    let drawings: Vec<u8> = lines[0]
        .split(',')
        .map(|l| l.parse::<u8>().unwrap())
        .collect();
    let mut boards: Vec<Board> = lines[1..]
        .chunks_exact(5)
        .map(|bl| {
            let board_lines: Vec<Mark> = bl
                .iter()
                .flat_map(|l| {
                    l.split_whitespace().map(|c| Mark {
                        number: c.parse::<u8>().unwrap(),
                        marked: false,
                    })
                })
                .collect();
            let grid: Grid<Mark> = Grid::from_vec(board_lines, 5);
            Board::new(grid)
        })
        .collect();

    println!("part 1");
    for &d in &drawings {
        if advance_boards(&mut boards, d) {
            break;
        }
    }

    println!("part 2");
    let mut last_won: Option<Board> = None;
    for &d in &drawings {
        if let Some(b) = advance_boards_last(&mut boards, d) {
            last_won = Some(b);
        }
    }

    if let Some(b) = last_won {
        print_board_win(&b);
    }
}

fn advance_boards(boards: &mut [Board], n: u8) -> bool {
    for b in boards {
        b.mark(n);
        if b.won.is_some() {
            print_board_win(&b);
            return true;
        }
    }

    return false;
}

fn advance_boards_last(boards: &mut [Board], n: u8) -> Option<Board> {
    let mut last_won: Option<Board> = None;
    for b in boards.iter_mut().filter(|b| b.won.is_none()) {
        b.mark(n);
        if b.won.is_some() {
            last_won = Some(b.clone());
        }
    }

    last_won
}

fn print_board_win(board: &Board) {
    if let Some((score, drawing)) = board.score() {
        let sum = board.sum_unmarked();
        println!("board completed on drawing {}: {:#?}", drawing, board);
        println!("sum: {}, drawing: {}, score: {}", sum, drawing, score);
    }
}
