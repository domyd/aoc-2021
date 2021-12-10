use std::collections::VecDeque;

#[allow(dead_code)]
pub fn run() {
    let input = include_str!("../inputs/10.txt");
    let scores = {
        let mut scores: Vec<_> = input
        .lines()
        .filter_map(|l| {
            parse_line(l)
                .ok()
                .map(|line| match line {
                    Line::Incomplete(cmp) => Some(cmp.chars().fold(0usize, |acc, c| {
                        (acc * 5)
                            + match c {
                                ')' => 1,
                                ']' => 2,
                                '}' => 3,
                                '>' => 4,
                                _ => panic!("illegal char"),
                            }
                    })),
                    _ => None,
                })
                .flatten()
        })
        .collect();

        scores.sort();
        scores
    };

    let score = scores[scores.len() / 2];
    println!("score: {}", score);
}

#[derive(Debug)]
struct ParseError {
    #[allow(dead_code)]
    illegal: char,
}

enum Line {
    Valid,
    Incomplete(String),
}

fn parse_line(line: &str) -> Result<Line, ParseError> {
    let mut stack: VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        match c {
            c @ ('(' | '[' | '{' | '<') => {
                stack.push_back(c);
            }
            c @ (')' | ']' | '}' | '>') => {
                if let Some(top) = stack.back() {
                    if closing_bracket(*top) == c {
                        stack.pop_back();
                    } else {
                        return Err(ParseError { illegal: c });
                    }
                }
            }
            c => panic!("illegal character {}", c),
        };
    }

    if stack.len() == 0 {
        Ok(Line::Valid)
    } else {
        Ok(Line::Incomplete(
            stack.iter().rev().map(|c| closing_bracket(*c)).collect(),
        ))
    }
}

fn closing_bracket(bracket: char) -> char {
    match bracket {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unsupported bracket {}", bracket),
    }
}
