use aoc_utils::PuzzleInput;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
const DAY: u8 = 21;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct Player {
    current_position: u16,
    score: usize,
}

fn parse_players(input: &PuzzleInput) -> (Player, Player) {
    input
        .lines()
        .iter()
        .map(|l| l.split(": ").nth(1).unwrap())
        .map(|v| v.parse::<u16>().unwrap())
        .map(|p| Player {
            current_position: p,
            score: 0,
        })
        .collect_tuple()
        .unwrap()
}

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn simulate_dice_game_a<F>(
    input: &PuzzleInput,
    win_score: usize,
    dice: Rc<RefCell<F>>,
    players: (Player, Player),
    current_player: usize,
) -> (Player, Player)
where
    F: FnMut() -> u16,
{
    if players.0.score >= win_score || players.1.score >= win_score {
        return players;
    }

    let mut p = if current_player == 0 {
        players.0
    } else {
        players.1
    };

    let mut pos = p.current_position;
    for _ in 0..3 {
        pos += dice.borrow_mut()();
    }

    while pos > 10 {
        pos -= 10;
    }

    p.current_position = pos;
    p.score += pos as usize;

    let new_player = (current_player + 1) % 2;
    let players = if current_player == 0 {
        (p, players.1)
    } else {
        (players.0, p)
    };
    simulate_dice_game_a(input, win_score, dice, players, new_player)
}

type SimluationCache = Rc<RefCell<HashMap<(usize, (Player, Player)), (usize, usize)>>>;

fn simulate_dice_game_b(
    input: &PuzzleInput,
    win_score: usize,
    players: (Player, Player),
    current_player: usize,
    cache: SimluationCache,
) -> (usize, usize) {
    if players.0.score >= win_score {
        return (1, 0);
    } else if players.1.score >= win_score {
        return (0, 1);
    }

    let mut players = players;
    let p = if current_player == 0 {
        &mut players.0
    } else {
        &mut players.1
    };

    let mut player_posibilities = vec![];
    let throw_posibilities = [1, 2, 3];

    for t1 in throw_posibilities.iter() {
        for t2 in throw_posibilities.iter() {
            for t3 in throw_posibilities.iter() {
                let mut pos = p.current_position + (t1 + t2 + t3);
                while pos > 10 {
                    pos -= 10;
                }

                player_posibilities.push(Player {
                    current_position: pos,
                    score: p.score + pos as usize,
                });
            }
        }
    }

    let new_player = (current_player + 1) % 2;
    player_posibilities
        .iter()
        .map(|p| {
            let players = if current_player == 0 {
                (*p, players.1)
            } else {
                (players.0, *p)
            };

            let cache_key = (current_player, players);
            if cache.borrow().contains_key(&cache_key) {
                *cache.borrow().get(&cache_key).unwrap()
            } else {
                let v = simulate_dice_game_b(input, win_score, players, new_player, cache.clone());
                cache.borrow_mut().insert(cache_key, v);
                v
            }
        })
        .fold((0, 0), |acc, v| (acc.0 + v.0, acc.1 + v.1))
}

fn solve_a(input: &PuzzleInput) -> usize {
    let mut dice = 1;
    let mut dice_rolls = 0;
    let dice = || {
        let v = dice;

        dice += 1;
        if dice >= 101 {
            dice = 1;
        }
        dice_rolls += 1;
        v
    };
    let players = parse_players(input);
    let result = simulate_dice_game_a(input, 1000, Rc::new(RefCell::new(dice)), players, 0);

    let looser = if result.0.score < result.1.score {
        &result.0
    } else {
        &result.1
    };

    dice_rolls * looser.score
}

fn solve_b(input: &PuzzleInput) -> usize {
    let players = parse_players(input);
    let cache = HashMap::new();
    let win_counts = simulate_dice_game_b(input, 21, players, 0, Rc::new(RefCell::new(cache)));

    std::cmp::max(win_counts.0, win_counts.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Player 1 starting position: 4\n\
                              Player 2 starting position: 8";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        // solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 739785);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 444356092776315);
    }
}
