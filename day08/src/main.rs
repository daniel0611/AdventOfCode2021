use aoc_utils::PuzzleInput;
use std::convert::TryInto;
const DAY: u8 = 8;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

struct SegmentDisplayMesurements {
    measurements: [String; 10],
    output: [String; 4],
}

impl SegmentDisplayMesurements {
    fn parse(input: &str) -> Self {
        let parts = input.split(" | ").collect::<Vec<_>>();
        let mut measurements = parts[0]
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let output = parts[1]
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        measurements.sort();

        SegmentDisplayMesurements {
            measurements: measurements.try_into().unwrap(),
            output: output.try_into().unwrap(),
        }
    }

    fn get_bits(s: &str) -> u8 {
        s.as_bytes()
            .iter()
            .fold(0u8, |acc, b| acc + (1 << (b - b'a') as usize))
    }

    fn get_number(&self) -> usize {
        let mut result = 0;
        let base: usize = 10;

        let mut one = 0u8;
        let mut four = 0u8;

        for measurement in &self.measurements {
            match measurement.len() {
                2 => one = Self::get_bits(measurement),
                4 => four = Self::get_bits(measurement),
                _ => (),
            }
        }

        for (i, measurement) in self.output.to_vec().iter().enumerate() {
            let v = Self::get_bits(measurement);
            let digit = match measurement.len() {
                2 => 1,
                4 => 4,
                3 => 7,
                7 => 8,

                5 => {
                    if (v & one).count_ones() == 2 {
                        3
                    } else if (v & four).count_ones() == 3 {
                        5
                    } else {
                        2
                    }
                }
                6 => {
                    if (v & one).count_ones() == 1 {
                        6
                    } else if (v & four).count_ones() == 3 {
                        0
                    } else {
                        9
                    }
                }

                _ => panic!("Invalid measurement: {}", measurement),
            };

            result += digit * base.pow((3 - i).try_into().unwrap());
        }

        result
    }

    fn count_easy_digits(&self) -> usize {
        self.output
            .iter()
            .filter(|m| matches!(m.len(), 2 | 3 | 4 | 7))
            .count()
    }
}

fn solve_a(input: &PuzzleInput) -> usize {
    let lines = input.lines();
    let segments = lines
        .iter()
        .map(|l| SegmentDisplayMesurements::parse(l))
        .collect::<Vec<_>>();

    segments.iter().map(|s| s.count_easy_digits()).sum()
}

fn solve_b(input: &PuzzleInput) -> usize {
    let lines = input.lines();
    let segments = lines
        .iter()
        .map(|l| SegmentDisplayMesurements::parse(l))
        .collect::<Vec<_>>();

    segments.iter().map(|s| s.get_number()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | \
fdgacbe cefdb cefbgd gcbe\n\
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | \
fcgedb cgb dgebacf gc\n\
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | \
cg cg fdcagb cbg\n\
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | \
efabcd cedba gadfec cb\n\
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | \
gecf egdcabf bgf bfgea\n\
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | \
gebdcfa ecba ca fadegcb\n\
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | \
cefg dcbef fcge gbcadfe\n\
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | \
ed bcgafe cdgba cbgef\n\
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | \
gbdfcae bgc cg cgb\n\
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | \
fgae cfgab fg bagce";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 26);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 61229);
    }
}
