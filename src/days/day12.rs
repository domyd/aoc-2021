use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Cave<'a> {
    Big(&'a str),
    Small(&'a str),
}

impl<'a> Cave<'a> {
    pub fn from(id: &'a str) -> Self {
        if id.chars().all(|c| c.is_uppercase()) {
            Cave::Big(id)
        } else {
            Cave::Small(id)
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    let input = include_str!("../inputs/12.txt");
    let input: Vec<_> = input
        .lines()
        .filter_map(|l| {
            l.split_once('-')
                .map(|(from, to)| (Cave::from(from), Cave::from(to)))
        })
        .collect();

    let nodes: HashSet<_> = input
        .iter()
        .copied()
        .flat_map(|(a, b)| [a, b].into_iter())
        .collect();
    let edges: HashMap<Cave, HashSet<Cave>> =
        HashMap::from_iter(nodes.iter().copied().map(|node| {
            let input = input
                .iter()
                .copied()
                .flat_map(|(a, b)| [(a, b), (b, a)].into_iter())
                .collect::<Vec<_>>();
            let edges = input
                .iter()
                .copied()
                .filter_map(|(from, to)| if from == node { Some(to) } else { None })
                .collect::<HashSet<_>>();
            (node, edges)
        }));

    // bfs
    let mut valid_paths = Vec::new();
    let mut q = VecDeque::from_iter(
        nodes
            .iter()
            .filter(|cave| match cave {
                Cave::Small(ref id) if id == &"start" => true,
                _ => false,
            })
            .map(|cave| vec![*cave]),
    );

    while let Some(path) = q.pop_back() {
        let visited = Vec::from_iter(path[..path.len() - 1].iter().copied());
        let cave = *path.last().unwrap();
        match cave {
            cave @ Cave::Small(c) if c == "start" => {
                if visited.contains(&cave) {
                    continue;
                }
            }
            Cave::Small(c) if c == "end" => {
                valid_paths.push(path);
                continue;
            }
            _ => {}
        };
        let cannot_visit_this_cave_anymore = {
            let mut histogram = HashMap::new();
            for cave in path.iter().filter(|c| matches!(c, Cave::Small(_))) {
                let entry = histogram.entry(*cave).or_insert(0);
                *entry += 1;
            }
            histogram.iter().any(|(_, v)| *v > 2)
                || histogram.iter().filter(|(_, v)| **v > 1).count() >= 2
        };
        if cannot_visit_this_cave_anymore {
            continue;
        }

        if let Some(set) = edges.get(&cave) {
            for neighbor_cave in set.iter() {
                let mut new_path = path.clone();
                new_path.push(*neighbor_cave);
                q.push_back(new_path);
            }
        }
    }

    eprintln!("valid paths: {}", valid_paths.len());
}
