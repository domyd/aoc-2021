use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};

#[allow(dead_code)]
pub fn run() {
    let input = include_str!("../inputs/14.txt");
    let (template, map) = {
        let mut map = HashMap::new();
        let mut lines = input.trim().lines().filter(|l| l.len() > 0);
        let template = lines.next().unwrap();
        for line in lines {
            let pair = line
                .split_once("->")
                .map(|(a, b)| (a.trim(), b.trim()))
                .unwrap();
            map.insert(pair.0, pair.1);
        }
        (template, map)
    };

    // Turn initial polymer into a frequency hashmap
    let mut elems: HashMap<&str, usize> = HashMap::new();
    for x in map.iter() {
        elems
            .entry(x.0)
            .or_insert(template.match_indices(x.0).count());
    }

    // Run the polymer expansion
    for _ in 1..=40 {
        let keys = elems
            .clone()
            .into_iter()
            .filter_map(|kvp| if kvp.1 > 0 { Some(kvp) } else { None })
            .collect::<Vec<_>>();
        for (k, count) in keys {
            if let Some((a, b)) = map.get(k).and_then(|x| x.chars().next()).map(|x| {
                let mut v = k.chars().collect::<Vec<_>>();
                v.insert(1, x);
                let (a, b): (String, String) =
                    ([v[0], v[1]].iter().collect(), [v[1], v[2]].iter().collect());
                (a, b)
            }) {
                *elems.get_mut(a.as_str()).unwrap() += count;
                *elems.get_mut(b.as_str()).unwrap() += count;
                elems.entry(k).and_modify(|x| *x = x.saturating_sub(count));
            }
        }
    }

    // Figure out the number of occurrences of each character
    let mut uniq_chars = HashMap::new();
    for ch in elems.keys().flat_map(|k| k.chars()).unique() {
        let frequencies = elems.iter().fold((0usize, 0usize), |acc, kvp| {
            let chars = kvp.0.chars().collect::<Vec<_>>();
            let val = kvp.1;
            let mut acc = acc;
            if chars[0] == ch {
                acc = (acc.0 + val, acc.1)
            }
            if chars[1] == ch {
                acc = (acc.0, acc.1 + val);
            }
            acc
        });
        uniq_chars.insert(ch, usize::max(frequencies.0, frequencies.1));
    }

    // Extract the most and least frequent characters from the
    let mut heap = BinaryHeap::new();
    for (c, n) in uniq_chars.into_iter() {
        heap.push((n, c));
    }
    let uniq_chars = heap.into_sorted_vec();
    let min = uniq_chars.first().unwrap();
    let max = uniq_chars.last().unwrap();

    eprintln!("min: {:?}, max: {:?}", min, max);
    eprintln!("result: {}", max.0 - min.0);
}
