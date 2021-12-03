use aoc_utils::PuzzleInput;
const DAY: u8 = 3;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn solve_a(input: &PuzzleInput) -> u32 {
    let lines = input.lines();
    let width = lines.first().unwrap().len() as u32;

    let base: u32 = 2;
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..width {
        let values = lines.iter().map(|line| line.chars().nth(i as usize).unwrap()).collect::<Vec<_>>();
        let zero_count = values.iter().filter(|&c| *c == '0').count();
        let one_count = values.iter().filter(|&c| *c == '1').count();

        if one_count > zero_count {
            // 1 is the most common bit here => add to gamma
            gamma += base.pow(width - i - 1);
        } else {
            // 0 is the most common bit here => add to epsilon
            epsilon += base.pow(width - i - 1);
        }
    }

    gamma * epsilon
}

fn solve_b(input: &PuzzleInput) -> u32 {
    let lines = input.lines();
    let oxygen_generator_rating = search_by_bit_criteria(&lines, true, 0);
    let co2_scrubber_rating = search_by_bit_criteria(&lines, false, 0);
    oxygen_generator_rating * co2_scrubber_rating
}

fn search_by_bit_criteria(lines: &Vec<String>, most_common: bool, index: usize) -> u32 {
    let mut zero_count = 0;
    let mut one_count = 0;

    for i in lines.iter() {
        let c = i.chars().nth(index).unwrap();
        if c == '0' {
            zero_count += 1;
        } else if c == '1' {
            one_count += 1;
        }
    }

    let valid_char = if most_common && one_count >= zero_count {
        '1'
    } else if most_common && one_count < zero_count {
        '0'
    } else if !most_common && one_count >= zero_count {
        '0'
    } else if !most_common && one_count < zero_count {
        '1'
    } else {
        panic!("Invalid");
    };

    let mut cleaned_lines = vec![];
    for line in lines.iter() {
        if line.chars().nth(index).unwrap() == valid_char {
            cleaned_lines.push(line.clone());
        }
    }

    if cleaned_lines.len() == 1 {
        let binary_value_str = cleaned_lines.first().unwrap();
        u32::from_str_radix(binary_value_str, 2).unwrap()
    } else {
        search_by_bit_criteria(&cleaned_lines, most_common, index + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(EXAMPLE_INPUT)), 198);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(EXAMPLE_INPUT)), 230);
    }
}
