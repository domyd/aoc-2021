#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Player {
    position: usize,
    score: usize,
    wins: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct GameState {
    players: [Player; 2],
}

impl GameState {
    pub fn advance(&self, player_idx: usize, roll: usize) -> GameState {
        let mut copy = *self;
        copy.players[player_idx].advance(roll);
        copy
    }
}

#[derive(Debug)]
struct Die {
    last_roll: usize,
    n_rolls: usize,
    faces: usize,
}

impl Player {
    pub fn advance(&mut self, roll: usize) {
        self.position = ((self.position + roll - 1) % 10) + 1;
        self.score += self.position;
    }
}

impl Die {
    pub fn new(faces: usize) -> Die {
        Die {
            last_roll: 0,
            n_rolls: 0,
            faces,
        }
    }

    pub fn roll(&mut self) -> usize {
        let next_roll = self.last_roll + 1;
        let next_roll = if next_roll > self.faces {
            next_roll % self.faces
        } else {
            next_roll
        };
        self.last_roll = next_roll;
        self.n_rolls += 1;
        next_roll
    }
}

#[allow(dead_code)]
fn part1(state: GameState) {
    let mut game = state;
    let target_score = 1000;
    let mut die = Die::new(10);
    let (mut winner, mut loser) = (None, None);

    for i in 0.. {
        let idx = i % 2;
        let rolls = (die.roll(), die.roll(), die.roll());
        let roll = rolls.0 + rolls.1 + rolls.2;
        game.players[idx].advance(roll);

        if game.players[idx].score >= target_score {
            winner = Some(game.players[idx]);
            loser = Some(game.players[(idx + 1) % 2]);
            break;
        }
    }

    eprintln!(
        "rolls: {}, winner: {:?}, loser: {:?}",
        die.n_rolls, winner, loser
    );

    eprintln!("result: {}", die.n_rolls * loser.unwrap().score);
}

#[allow(dead_code)]
fn part2(state: GameState) {
    let target_score = 21;
    let result = run_quantum_game((state, 1), 0, target_score);
    let winning_player = if result.wins[0] > result.wins[1] {
        1
    } else {
        2
    };
    eprintln!("player {} wins - result: {:?}", winning_player, &result);
}

#[derive(Debug)]
struct GameResult {
    wins: [usize; 2],
    played: usize,
}

static DICE_MAP: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

fn run_quantum_game(state: (GameState, usize), depth: usize, target_score: usize) -> GameResult {
    let (cur, prev) = (depth % 2, (depth + 1) % 2);
    let result = if state.0.players[prev].score >= target_score {
        let mut wins = [0usize; 2];
        wins[prev] = state.1;
        GameResult {
            played: state.1,
            wins,
        }
    } else {
        let (state, group) = (state.0, state.1);
        let mut total = GameResult {
            played: 0,
            wins: [0, 0],
        };
        for i in 3..=9 {
            let n = DICE_MAP[i];
            let result = run_quantum_game((state.advance(cur, i), n), depth + 1, target_score);
            total.played += group * result.played;
            total.wins[0] += group * result.wins[0];
            total.wins[1] += group * result.wins[1];
        }
        total
    };

    result
}

#[allow(dead_code)]
pub fn run() {
    let (p1, p2) = {
        let input = include_str!("../inputs/21.txt");
        let lines = input.lines().collect::<Vec<_>>();
        let p1 = lines[0]
            .trim_start_matches("Player 1 starting position: ")
            .parse()
            .unwrap();
        let p2 = lines[1]
            .trim_start_matches("Player 2 starting position: ")
            .parse()
            .unwrap();

        (
            Player {
                position: p1,
                score: 0,
                wins: 0,
            },
            Player {
                position: p2,
                score: 0,
                wins: 0,
            },
        )
    };

    let state = GameState {
        players: [p1.clone(), p2.clone()],
    };

    part1(state.clone());
    part2(state.clone());
}
