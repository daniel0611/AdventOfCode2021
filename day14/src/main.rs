use aoc_utils::PuzzleInput;
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
                let src = parts.next().unwrap().to_string();
                let replacement_char = parts.next().unwrap().chars().next().unwrap();
                (src, replacement_char)
            })
            .collect::<HashMap<_, _>>();

        let mut current: HashMap<String, usize> = HashMap::new();
        for i in 0..template.len() - 1 {
            let s = &template[i..=i + 1];
            *current.entry(s.to_string()).or_insert(0) += 1;
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
            let middle_char = self.rules[key];

            let first_key = format!("{}{}", key.chars().next().unwrap(), middle_char);
            *output.entry(first_key.clone()).or_default() += count;

            let second_key = format!("{}{}", middle_char, key.chars().last().unwrap());
            *output.entry(second_key.clone()).or_default() += count;
        }

        self.current = output;
    }

    fn calculate_score(&self) -> usize {
        let mut char_map: HashMap<char, usize> = HashMap::new();

        for (chars, value) in self.current.iter() {
            let c = chars.chars().nth(1).unwrap();
            *char_map.entry(c).or_default() += value;
        }

        let char_counts = char_map.iter().map(|(_, count)| *count).collect::<Vec<_>>();
        char_counts.iter().max().unwrap() - char_counts.iter().min().unwrap()
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
