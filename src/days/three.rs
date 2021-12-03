pub fn three() {
    let input = include_str!("../inputs/three.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let nbits = lines[0].len();

    part1(&lines, nbits);

    let numbers: Vec<usize> = lines
        .iter()
        .map(|l| usize::from_str_radix(*l, 2).unwrap())
        .collect();

    let oxygen = binary_search(&numbers, nbits, Mode::Most).unwrap();
    let co2 = binary_search(&numbers, nbits, Mode::Least).unwrap();

    println!(
        "oxygen = {}, co2 = {}, life support = {}",
        oxygen,
        co2,
        oxygen * co2
    );
}

fn part1(lines: &[&str], nbits: usize) {
    let threshold = lines.len() / 2;
    let mut onebits = vec![0usize; nbits];

    for l in lines.iter() {
        for (i, _) in l.chars().enumerate().filter(|(_, c)| *c == '1') {
            onebits[i] += 1
        }
    }

    let gamma_bits: Vec<u8> = onebits
        .iter()
        .map(|n| if *n as usize > threshold { 1 } else { 0 })
        .collect();
    let epsilon_bits: Vec<u8> = gamma_bits
        .iter()
        .map(|n| if *n == 1 { 0 } else { 1 })
        .collect();

    let gamma_str: String = gamma_bits
        .iter()
        .map(|n| if *n == 1 { '1' } else { '0' })
        .collect();
    let epsilon_str: String = epsilon_bits
        .iter()
        .map(|n| if *n == 1 { '1' } else { '0' })
        .collect();

    let gamma = usize::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_str, 2).unwrap();

    println!(
        "gamma = {}, epsilon = {}, product = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

enum Mode {
    Most,
    Least,
}

fn binary_search(numbers: &[usize], nbits: usize, mode: Mode) -> Option<usize> {
    let mut filtered: Vec<usize> = Vec::from(numbers);
    let mut bitpos = nbits - 1;
    loop {
        let (ones, zeros): (Vec<usize>, Vec<usize>) =
            filtered.iter().partition(|x| get_bit_at(**x, bitpos));
        filtered = match mode {
            Mode::Most => {
                if ones.len() >= zeros.len() {
                    ones
                } else {
                    zeros
                }
            }
            Mode::Least => {
                if ones.len() >= zeros.len() {
                    zeros
                } else {
                    ones
                }
            }
        };

        if filtered.len() == 1 {
            break Some(filtered[0]);
        }
        if bitpos == 0 {
            break None;
        }

        bitpos -= 1;
    }
}

fn get_bit_at(input: usize, n: usize) -> bool {
    input & (1 << n) != 0
}
