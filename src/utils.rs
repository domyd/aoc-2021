use std::{collections::HashMap, fmt::Display};

pub fn print_hashmap_grid<V: Display>(map: &HashMap<(isize, isize), V>) {
    if map.is_empty() {
        eprintln!("map is empty");
        return;
    }

    let keys: Vec<_> = map.keys().collect();
    let dim_y = keys.iter().max_by_key(|f| f.0).map(|f| f.0).unwrap() + 1;
    let dim_x = keys.iter().max_by_key(|f| f.1).map(|f| f.1).unwrap() + 1;
    let longest_v = map.values().map(|v| v.to_string().len()).max().unwrap();
    let empty = core::iter::repeat(' ').take(longest_v).collect::<String>();

    let mut out = String::new();
    for y in 0..dim_y {
        let row: Vec<_> = (0..dim_x)
            .map(|x| map.get(&(y, x)).map(|v| v.to_string()))
            .collect();

        for el in row.iter() {
            let str = if let Some(s) = el { &s } else { &empty };
            out.push_str(format!("{:>width$} ", str, width = longest_v).as_ref());
        }

        out.push('\n');
    }

    eprintln!("{}", out);
}

pub fn vec_into_hashmap<V: Copy>(vec: Vec<Vec<V>>) -> (HashMap<(isize, isize), V>, usize, usize) {
    let rows = vec.len();
    let cols = vec.iter().map(|l| l.len()).max().unwrap_or_default();

    let mut map = HashMap::new();
    for y in 0..rows {
        for x in 0..cols {
            map.insert((y as isize, x as isize), vec[y][x]);
        }
    }

    (map, rows, cols)
}
