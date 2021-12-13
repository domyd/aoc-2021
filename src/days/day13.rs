#[derive(Clone, Copy, Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

#[allow(dead_code)]
pub fn run() {
    let input = include_str!("../inputs/13.txt");
    let mut numbers: Vec<(usize, usize)> = Vec::new();
    let mut folds: Vec<Fold> = Vec::new();
    for line in input.trim().split('\n') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((a, b)) = line.split_once(',') {
            numbers.push((a.parse().unwrap(), b.parse().unwrap()));
        } else {
            let fold = line.split(' ').skip(2).next().unwrap();
            let fold = fold.split_once('=').unwrap();
            let fold = (fold.0, fold.1.parse::<usize>().unwrap());
            let fold = if fold.0 == "x" {
                Fold::X(fold.1)
            } else {
                Fold::Y(fold.1)
            };
            folds.push(fold);
        }
    }

    let (mut rows, mut cols) = {
        let x = numbers.iter().map(|n| n.0).max().unwrap();
        let y = numbers.iter().map(|n| n.1).max().unwrap();
        (y, x)
    };

    for fold in folds.iter() {
        if let Fold::X(x) = *fold {
            let right: Vec<_> = numbers.iter().filter(|n| n.0 > x).copied().collect();
            for pair in right {
                let index = numbers.iter().position(|x| *x == pair).unwrap();
                numbers.remove(index);
                let pair = (x - (pair.0 - x), pair.1);
                if let Some(_) = numbers.iter().position(|x| *x == pair) {
                    continue;
                }
                numbers.push(pair);
            }

            cols = cols / 2
        } else if let Fold::Y(y) = *fold {
            let bottom: Vec<_> = numbers.iter().filter(|n| n.1 > y).copied().collect();
            for pair in bottom {
                let index = numbers.iter().position(|x| *x == pair).unwrap();
                numbers.remove(index);
                let pair = (pair.0, y - (pair.1 - y));
                if let Some(_) = numbers.iter().position(|x| *x == pair) {
                    continue;
                }
                numbers.push(pair);
            }

            rows = rows / 2
        }
    }

    eprintln!("visible dots: {}", numbers.len());

    for y in 0..rows {
        let mut line = String::new();
        for x in 0..cols {
            if let Some(_) = numbers.iter().position(|n| *n == (x, y)) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        eprintln!("{}", line);
    }
}
