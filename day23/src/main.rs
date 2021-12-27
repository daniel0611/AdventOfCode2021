use aoc_utils::PuzzleInput;
use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::rc::Rc;
const DAY: u8 = 23;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn from_char(c: char) -> Option<Amphipod> {
        match c {
            'A' => Some(Amphipod::Amber),
            'B' => Some(Amphipod::Bronze),
            'C' => Some(Amphipod::Copper),
            'D' => Some(Amphipod::Desert),
            _ => None,
        }
    }

    fn get_energy_per_step(&self) -> u32 {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }

    fn get_target_room_idx(&self) -> usize {
        match self {
            Amphipod::Amber => 0,
            Amphipod::Bronze => 1,
            Amphipod::Copper => 2,
            Amphipod::Desert => 3,
        }
    }

    fn get_room_type(room_idx: usize) -> Amphipod {
        match room_idx {
            0 => Amphipod::Amber,
            1 => Amphipod::Bronze,
            2 => Amphipod::Copper,
            3 => Amphipod::Desert,
            _ => panic!("Invalid room index"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct AmphipodState {
    rooms: [Vec<Amphipod>; 4],
    hallway: [Option<Amphipod>; 11],
    current_energy: u32,
    room_max_count: usize,
}

impl AmphipodState {
    fn parse(input: &PuzzleInput) -> AmphipodState {
        let lines = input.lines();
        let hallway = lines[1][1..12]
            .chars()
            .map(Amphipod::from_char)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let rooms = (0..4)
            .map(|i| Self::parse_room(&lines, i))
            .collect::<Vec<_>>();
        let room_size = rooms[0].len();

        AmphipodState {
            rooms: rooms.try_into().unwrap(),
            hallway,
            current_energy: 0,
            room_max_count: room_size,
        }
    }

    fn parse_extended(input: &PuzzleInput) -> AmphipodState {
        let input_lines = input.lines();
        let mut new_lines = vec![];
        new_lines.extend(input_lines[0..=2].iter().cloned());
        new_lines.push("  #D#C#B#A#".to_string());
        new_lines.push("  #D#B#A#C#".to_string());
        new_lines.extend(input_lines[3..].iter().cloned());

        for l in new_lines.iter() {
            println!("{}", l);
        }

        Self::parse(&PuzzleInput::new(new_lines.join("\n")))
    }

    fn parse_room(lines: &[String], room_idx: usize) -> Vec<Amphipod> {
        let amphipods = lines[2..lines.len() - 1]
            .iter()
            .rev()
            .map(|line| {
                line.chars()
                    .skip(1)
                    .nth(Self::room_idx_to_hallway_idx(room_idx))
                    .unwrap()
            })
            .flat_map(Amphipod::from_char)
            .collect::<Vec<_>>();

        amphipods
    }

    fn room_idx_to_hallway_idx(room_number: usize) -> usize {
        (room_number + 1) * 2
    }

    fn get_minimum_energy(
        current_step: AmphipodState,
        cache: Rc<RefCell<HashMap<AmphipodState, u32>>>,
    ) -> u32 {
        if current_step.is_finished() {
            return current_step.current_energy;
        }

        let mut steps = vec![];
        steps.extend(current_step.get_room_to_hallway_steps());
        steps.extend(current_step.get_hallway_to_room_steps());
        steps
            .iter()
            .map(|step| {
                if cache.borrow().contains_key(step) {
                    *cache.borrow().get(step).unwrap()
                } else {
                    let min = Self::get_minimum_energy(step.clone(), cache.clone());
                    cache.borrow_mut().insert(step.clone(), min);
                    min
                }
            })
            .min()
            .unwrap_or(u32::max_value())
    }

    fn get_room_to_hallway_steps(&self) -> Vec<AmphipodState> {
        let mut steps = vec![];

        for (room_idx, room) in self.rooms.iter().enumerate() {
            if room.is_empty() {
                // No amphipods in room left
                continue;
            }

            if room
                .iter()
                .all(|amp| *amp == Amphipod::get_room_type(room_idx))
            {
                // All amphipods in this room are already in the correct room
                continue;
            }

            let start_hallway_pos = Self::room_idx_to_hallway_idx(room_idx);
            for end_pos in 0..self.hallway.len() {
                if end_pos == 2 || end_pos == 4 || end_pos == 6 || end_pos == 8 {
                    // Skip places before rooms, since we aren't allowed to stop there
                    continue;
                }

                if self.hallway_can_move(start_hallway_pos, end_pos) {
                    let mut state = self.clone();
                    let additional_steps = self.room_max_count - state.rooms[room_idx].len() + 1;
                    let amp = state.rooms[room_idx].pop();
                    state.hallway[end_pos] = amp.clone();
                    state.current_energy += Self::calculate_energy(
                        &amp.unwrap(),
                        start_hallway_pos,
                        end_pos,
                        additional_steps,
                    );

                    steps.push(state);
                }
            }
        }

        steps
    }

    fn get_hallway_to_room_steps(&self) -> Vec<AmphipodState> {
        let mut steps = vec![];

        for (start_pos, amp) in self.hallway.iter().enumerate() {
            if let Some(amp) = amp {
                let amp_target_room_idx = amp.get_target_room_idx();
                let room = &self.rooms[amp_target_room_idx];
                let hallway_end_idx = Self::room_idx_to_hallway_idx(amp_target_room_idx);

                // Ensure that there is not some other amphipod in the target room that still needs to leave
                if !room.is_empty() && room.iter().any(|a| *a != *amp) {
                    continue;
                }

                if self.hallway_can_move(start_pos, hallway_end_idx) {
                    let mut state = self.clone();
                    let additional_steps =
                        self.room_max_count - state.rooms[amp_target_room_idx].len();
                    state.current_energy +=
                        Self::calculate_energy(amp, start_pos, hallway_end_idx, additional_steps);
                    state.hallway[start_pos] = None;
                    state.rooms[amp_target_room_idx].push(amp.clone());

                    steps.push(state);
                }
            }
        }

        steps
    }

    fn hallway_can_move(&self, start_pos: usize, end_pos: usize) -> bool {
        let min_pos = cmp::min(start_pos, end_pos);
        let max_pos = cmp::max(start_pos, end_pos);
        (min_pos..=max_pos)
            .filter(|p| *p != start_pos)
            .all(|pos| self.hallway[pos].is_none())
    }

    fn calculate_energy(
        amp: &Amphipod,
        start_pos: usize,
        end_pos: usize,
        additional: usize,
    ) -> u32 {
        let min_pos = cmp::min(start_pos, end_pos);
        let max_pos = cmp::max(start_pos, end_pos);
        let steps = max_pos - min_pos + additional;
        steps as u32 * amp.get_energy_per_step()
    }

    fn is_finished(&self) -> bool {
        let hallway_empty = self.hallway.iter().all(|amp| amp.is_none());

        let rooms_correct =
            (0..4)
                .map(|i| (i, Amphipod::get_room_type(i)))
                .all(|(room_idx, room_type)| {
                    self.rooms[room_idx].len() == self.room_max_count
                        && self.rooms[room_idx].iter().all(|amp| *amp == room_type)
                });

        hallway_empty && rooms_correct
    }
}

fn solve_a(input: &PuzzleInput) -> u32 {
    let start_state = AmphipodState::parse(input);
    AmphipodState::get_minimum_energy(start_state, Rc::new(RefCell::new(HashMap::new())))
}

fn solve_b(input: &PuzzleInput) -> u32 {
    let start_state = AmphipodState::parse_extended(input);
    AmphipodState::get_minimum_energy(start_state, Rc::new(RefCell::new(HashMap::new())))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "#############\n#...........#\n###B#C#B#D###\n  #A#D#C#A#\n  #########";

    // #[test]
    // fn test_no_panic() {
    //     let input = PuzzleInput::get_input(DAY);
    //     solve_a(&input);
    //     solve_b(&input);
    // }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 12521);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 44169);
    }
}
