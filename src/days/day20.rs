use crate::utils::{print_hashmap_grid, vec_into_hashmap};
use std::collections::HashMap;

#[derive(Clone)]
struct Image {
    pixels: HashMap<(isize, isize), char>,
    size: Vec2,
    origin: Vec2,
    infinity_pixel: char,
}

impl Image {
    pub fn pixel_at(&self, pos: &(isize, isize)) -> char {
        if let Some(&px) = self.pixels.get(&pos) {
            // Return pixel value from storage
            px
        } else {
            // If out-of-bounds, return pixel value of current cell
            self.infinity_pixel
        }
    }
}

#[derive(Clone, Copy)]
struct Vec2 {
    x: isize,
    y: isize,
}

#[allow(dead_code)]
pub fn run() {
    let (algorithm, mut image) = {
        let input = include_str!("../inputs/20.txt");
        let mut lines = input.lines();

        let algo = lines.next().unwrap().chars().collect::<Vec<_>>();
        let chars: Vec<Vec<char>> = lines
            .filter_map(|l| {
                if !l.is_empty() {
                    Some(l.chars().collect())
                } else {
                    None
                }
            })
            .collect();
        let (pixels, size_x, size_y) = vec_into_hashmap(chars);
        let image = Image {
            pixels,
            size: Vec2 {
                x: size_x as isize,
                y: size_y as isize,
            },
            origin: Vec2 { x: 0, y: 0 },
            infinity_pixel: '.',
        };

        (algo, image)
    };

    (1..=50).for_each(|_| {
        image = add_px_frame(&image, 1);
        image = enhance_image(&image, &algorithm);
    });

    print_image(&image);

    let lit_pixels = image.pixels.iter().filter(|(_, c)| **c == '#').count();
    eprintln!("lit pixels: {}", lit_pixels);
}

fn enhance_image(input: &Image, algorithm: &[char]) -> Image {
    let kernel: [(isize, isize); 9] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut output = input.clone();

    // Run kernel on input image and write to output image
    for kvp in input.pixels.iter() {
        let pos @ (y, x) = *kvp.0;
        let neighbors: [char; 9] = kernel.map(|(dy, dx)| {
            let dp = (y + dy, x + dx);
            input.pixel_at(&dp)
        });
        let index = pixels_to_num(neighbors);
        *output.pixels.get_mut(&pos).unwrap() = algorithm[index];
    }

    // Swap infinity pixel
    if algorithm[0] == '#' && output.infinity_pixel == '.' {
        output.infinity_pixel = '#';
    } else if algorithm[511] == '.' && output.infinity_pixel == '#' {
        output.infinity_pixel = '.'
    }

    output
}

fn add_px_frame(image: &Image, distance: usize) -> Image {
    let mut image = image.clone();
    let distance = distance as isize;
    let (y0, y1) = (
        image.origin.y - distance,
        image.origin.y + image.size.y as isize + distance,
    );
    let (x0, x1) = (
        image.origin.x - distance,
        image.origin.x + image.size.x as isize + distance,
    );

    for y in y0..y1 {
        image.pixels.entry((y, x0)).or_insert(image.infinity_pixel);
        image
            .pixels
            .entry((y, x1 - 1))
            .or_insert(image.infinity_pixel);
    }
    for x in x0..x1 {
        image.pixels.entry((y0, x)).or_insert(image.infinity_pixel);
        image
            .pixels
            .entry((y1 - 1, x))
            .or_insert(image.infinity_pixel);
    }

    image.size.x += 2 * distance;
    image.size.y += 2 * distance;
    image.origin.x -= distance;
    image.origin.y -= distance;
    image
}

fn pixels_to_num(pixels: [char; 9]) -> usize {
    let u8s = pixels.map(|p| if p == '#' { b'1' } else { b'0' });
    let binary_str = std::str::from_utf8(&u8s).unwrap();
    usize::from_str_radix(&binary_str, 2).unwrap()
}

fn print_image(image: &Image) {
    print_hashmap_grid(&image.pixels);
}
