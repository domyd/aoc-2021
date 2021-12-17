use std::vec;

#[derive(Debug)]
struct Trajectory {
    path: Vec<(i32, i32)>,
    initial_v: (u32, i32),
}

#[allow(dead_code)]
pub fn run() {
    let (tx, ty) = {
        let input = include_str!("../inputs/17.txt");
        let input = input.trim().split(" ").skip(2).collect::<String>();
        let (x, y) = input.split_once(',').unwrap();
        let x = x.split_once('=').unwrap().1;
        let y = y.split_once('=').unwrap().1;
        let xy = vec![x, y]
            .into_iter()
            .map(|range| {
                range
                    .split_once("..")
                    .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
                    .unwrap()
            })
            .collect::<Vec<(i32, i32)>>();
        let x = xy[0];
        let y = xy[1];
        (x, y)
    };

    let min_x_vel = (0u32..)
        .skip_while(|n| (*n as i32 * (*n as i32 + 1)) / 2 < tx.0)
        .next()
        .unwrap();

    let mut trajectories = Vec::new();

    // Shot always passes through y=0 at velocity -vy0, and if -vy0 is smaller
    // than the lower y bound of the box, it will always pass through.
    let y_upper = ty.0.abs();

    for x_vel in min_x_vel..=1000u32 {
        for y_vel in ty.0..=y_upper {
            let starting_velocity = (x_vel, y_vel);
            let mut velocity = starting_velocity;
            let mut path = vec![(0i32, 0i32)];

            if let Some(trajectory) = loop {
                let prev_position = *path.last().unwrap();
                let position = (
                    prev_position.0 + velocity.0 as i32,
                    prev_position.1 + velocity.1,
                );
                path.push(position);
                velocity = (velocity.0.saturating_sub(1), velocity.1 - 1);

                if position.0 >= tx.0
                    && position.0 <= tx.1
                    && position.1 >= ty.0
                    && position.1 <= ty.1
                {
                    break Some(Trajectory {
                        path: path.clone(),
                        initial_v: starting_velocity,
                    });
                } else if position.0 > tx.1 || position.1 < ty.0 {
                    // we're never, ever, ever, gonna hiiit the targeet
                    break None;
                }
            } {
                trajectories.push(trajectory);
            }
        }
    }

    let velocities = trajectories
        .into_iter()
        .map(|t| t.initial_v)
        .collect::<Vec<_>>();
    eprintln!(
        "{} different velocities hit the target {:?} {:?}",
        velocities.len(),
        tx,
        ty
    );
}
