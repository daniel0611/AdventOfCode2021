use aoc_utils::PuzzleInput;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
const DAY: u8 = 14;

struct Polymer {
    current: HashMap<String, usize>,
    rules: HashMap<String, char>,
}

impl Polymer {
    fn parse(input: &PuzzleInput) -> Polymer {
        let mut parts = input.raw_input.split("\n\n");
        let template = parts.next().unwrap();
        let rules_string = parts.next().unwrap();
        let rules = rules_string
            .split('\n')
            .map(|line| {
                let mut parts = line.split(" -> ");
                let left = parts.next().unwrap();
                let right = parts.next().unwrap();
                (left.to_string(), right.chars().next().unwrap())
            })
            .collect::<HashMap<_, _>>();

        let mut current = HashMap::new();
        for i in 0..template.len() - 1 {
            let s = &template[i..=i + 1];
            assert_eq!(s.len(), 2);
            if current.contains_key(s) {
                current.insert(s.to_string(), current[s] + 1);
            } else {
                current.insert(s.to_string(), 1);
            }
        }

        Polymer { current, rules }
    }

    fn execute_steps(&mut self, steps: usize) {
        for _ in 0..steps {
            self.execute_step();
        }
    }

    fn execute_step(&mut self) {
        let mut output = HashMap::new();

        for (key, count) in self.current.iter() {
            if !self.rules.contains_key(key) {
                continue;
            }
            let middle_char = self.rules[key];

            let mut first_key = String::default();
            first_key.push(key.chars().next().unwrap());
            first_key.push(middle_char);
            if output.contains_key(&first_key) {
                let new_value = output[&first_key] + count;
                output.insert(first_key, new_value);
            } else {
                output.insert(first_key, *count);
            }

            let mut second_key = String::default();
            second_key.push(middle_char);
            second_key.push(key.chars().last().unwrap());
            if output.contains_key(&second_key) {
                let new_value = output[&second_key] + count;
                output.insert(second_key, new_value);
            } else {
                output.insert(second_key, *count);
            }
        }

        self.current = output;
    }

    fn calculate_score(&self) -> usize {
        let mut char_map: HashMap<char, usize> = HashMap::new();

        for (chars, value) in self.current.iter() {
            let c = chars.chars().nth(1).unwrap();
            match char_map.entry(c) {
                Entry::Occupied(mut e) => {
                    e.insert(e.get() + value);
                }
                Entry::Vacant(e) => {
                    e.insert(*value);
                }
            };
        }

        let mut char_counts = char_map.iter().collect::<Vec<_>>();
        char_counts.sort_by(|a, b| a.1.cmp(b.1));
        char_counts.last().unwrap().1 - char_counts.first().unwrap().1
    }
}

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn solve_a(input: &PuzzleInput) -> usize {
    let mut polymer = Polymer::parse(input);
    polymer.execute_steps(10);
    polymer.calculate_score()
}

fn solve_b(input: &PuzzleInput) -> usize {
    let mut polymer = Polymer::parse(input);
    polymer.execute_steps(40);
    polymer.calculate_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "NNCB\n\
    \n\
    CH -> B\n\
    HH -> N\n\
    CB -> H\n\
    NH -> C\n\
    HB -> C\n\
    HC -> B\n\
    HN -> C\n\
    NN -> C\n\
    BH -> H\n\
    NC -> B\n\
    NB -> B\n\
    BN -> B\n\
    BB -> N\n\
    BC -> B\n\
    CC -> N\n\
    CN -> C";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 1588);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 2188189693529);
    }
}
