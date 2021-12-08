use std::collections::{HashMap, HashSet};

struct Segment {
    map: HashMap<usize, HashSet<char>>,
}

enum Digit {
    Known(u8),
    SixLen,
    FiveLen,
    SevenLen,
}

struct Pattern<'a> {
    signal: &'a str,
    digit: Digit,
}

impl<'a> Pattern<'a> {
    fn new(signal: &'a str) -> Self {
        Pattern {
            signal,
            digit: match signal.len() {
                2 => Digit::Known(1),
                3 => Digit::Known(7),
                4 => Digit::Known(4),
                5 => Digit::FiveLen,
                6 => Digit::SixLen,
                7 => Digit::SevenLen,
                _ => unreachable!(),
            },
        }
    }
}

impl Segment {
    pub fn from_pattern(pattern: &[&str]) -> Self {
        let patterns: Vec<Pattern> = pattern.into_iter().map(|s| Pattern::new(s)).collect();

        // Prepare segment map, which initially contains all possible combinations of segments
        let all_chars: HashSet<char> = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
        let mut map: HashMap<usize, HashSet<char>> = HashMap::new();
        (0..7).for_each(|i| {
            map.insert(i, all_chars.clone());
        });

        // Solve patterns with known digits first
        patterns
            .iter()
            .filter_map(|p| match p.digit {
                Digit::Known(d) => Some((p.signal, Self::segments_for_digit(d))),
                _ => None,
            })
            .for_each(|(signal, active_segments)| {
                Self::solve_signal(&mut map, signal, &active_segments);
            });

        // Next up, six length patterns (digits 0, 6, 9)
        {
            // Take the intersection of all six-length patterns and apply it with the
            // set of active segments for that pattern
            let active_segments: HashSet<usize> = HashSet::from([0, 1, 5, 6]);
            let signal = {
                let mut set = all_chars.clone();
                patterns
                    .iter()
                    .filter_map(|p| match p.digit {
                        Digit::SixLen => Some(HashSet::from_iter(p.signal.chars())),
                        _ => None,
                    })
                    .for_each(|chars| {
                        set = set.intersection(&chars).copied().collect();
                    });
                String::from_iter(set.iter())
            };

            Self::solve_signal(&mut map, &signal, &active_segments);
        }

        // No need to also solve with five length patterns
        // (unless the following assert triggers)

        assert!(
            map.values().all(|v| v.len() == 1),
            "no definitive mapping found (yet)"
        );

        Self { map }
    }

    pub fn decode(&self, signal: &str) -> Option<u8> {
        let segments: HashSet<usize> = signal
            .chars()
            .filter_map(|c| {
                self.map.iter().find_map(|(i, s)| {
                    if s.len() == 1 && s.contains(&c) {
                        Some(i)
                    } else {
                        None
                    }
                })
            })
            .copied()
            .collect();
        Self::digit_for_segments(&segments)
    }

    fn solve_signal(
        map: &mut HashMap<usize, HashSet<char>>,
        signal: &str,
        active_segments: &HashSet<usize>,
    ) {
        let inactive_segments = Self::invert_segments(active_segments);

        // Disassociate active signals from segments are aren't lit up
        for i in inactive_segments.iter() {
            signal.chars().for_each(|c| {
                map.entry(*i).and_modify(|set| {
                    set.remove(&c);
                });
            });
        }

        // Segments that _are_ lit up cannot map to signals other than the active ones
        // i.e. disassociate inactive signals from segments that are lit up
        for i in active_segments.iter() {
            let chars: Vec<char> = signal.chars().collect();
            map.entry(*i)
                .and_modify(|set| set.retain(|c| chars.contains(c)));
        }
    }

    fn digit_for_segments(segments: &HashSet<usize>) -> Option<u8> {
        for d in 0..10 {
            let segments_for_digit: HashSet<_> =
                Self::segments_for_digit(d).iter().copied().collect();
            if segments == &segments_for_digit {
                return Some(d);
            }
        }

        return None;
    }

    fn segments_for_digit(digit: u8) -> HashSet<usize> {
        match digit {
            0 => HashSet::from([0, 1, 2, 4, 5, 6]),
            1 => HashSet::from([2, 5]),
            2 => HashSet::from([0, 2, 3, 4, 6]),
            3 => HashSet::from([0, 2, 3, 5, 6]),
            4 => HashSet::from([1, 2, 3, 5]),
            5 => HashSet::from([0, 1, 3, 5, 6]),
            6 => HashSet::from([0, 1, 3, 4, 5, 6]),
            7 => HashSet::from([0, 2, 5]),
            8 => HashSet::from([0, 1, 2, 3, 4, 5, 6]),
            9 => HashSet::from([0, 1, 2, 3, 5, 6]),
            _ => unreachable!(),
        }
    }

    fn invert_segments(segments: &HashSet<usize>) -> HashSet<usize> {
        let set: HashSet<usize> = (0..7).collect();
        set.difference(&segments).into_iter().cloned().collect()
    }
}

pub fn eight() {
    let input = include_str!("../inputs/08.txt");
    let input: Vec<(Vec<&str>, Vec<&str>)> = input
        .lines()
        .filter_map(|l| {
            l.split_once('|').map(|(s, o)| {
                (
                    s.split_whitespace().collect(),
                    o.split_whitespace().collect(),
                )
            })
        })
        .collect();

    let numbers: Vec<usize> = input
        .iter()
        .map(|(signals, output)| {
            let segment = Segment::from_pattern(signals);
            let digits: Vec<usize> = output
                .iter()
                .filter_map(|s| segment.decode(s).map(|n| n as usize))
                .collect();
            let number = digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3];
            number
        })
        .collect();

    let sum: usize = numbers.iter().sum();
    println!("output total: {}", sum);
}
