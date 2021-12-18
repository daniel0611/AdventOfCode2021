use aoc_utils::PuzzleInput;
use std::rc::Rc;

const DAY: u8 = 18;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

enum NumberEntry {
    Value(Rc<NumberEntry>, Rc<NumberEntry>),
    Pair(u16),
}

type ExplodeIntermediate = (Option<Rc<NumberEntry>>, Option<u16>, Option<u16>);

impl NumberEntry {
    fn get_value(&self) -> Option<u16> {
        if let NumberEntry::Pair(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    fn parse(s: &str) -> Rc<NumberEntry> {
        if s.starts_with('[') {
            // Pair
            let left_str = Self::substring_pair_component(s, 1); // skip [
            let right_str = Self::substring_pair_component(s, 1 + left_str.len() + 1); // [ content and ,

            let left = Self::parse(&left_str);
            let right = Self::parse(&right_str);
            Rc::new(NumberEntry::Value(left, right))
        } else {
            // Plain number
            let v = s.parse().unwrap();
            Rc::new(NumberEntry::Pair(v))
        }
    }

    fn substring_pair_component(s: &str, start_index: usize) -> String {
        let mut end_index = start_index;
        let mut depth = 0;
        while end_index < s.len() {
            if s.chars().nth(end_index).unwrap() == '[' {
                depth += 1;
            } else if s.chars().nth(end_index).unwrap() == ']' {
                depth -= 1;
            }
            if depth == 0 {
                break;
            }
            end_index += 1;
        }
        s[start_index..=end_index].to_string()
    }

    fn explode(n: &Rc<NumberEntry>, d: usize) -> Option<ExplodeIntermediate> {
        if let NumberEntry::Value(left, right) = &**n {
            if d >= 4 {
                return Some((None, left.get_value(), right.get_value()));
            }

            // d <= 3

            if let Some((new_left, left_value, right_value)) = Self::explode(left, &d + 1) {
                let (new_r, _) = Self::add_to_left(right, right_value);
                let value = if let Some(new_left) = new_left {
                    Rc::new(NumberEntry::Value(new_left, new_r))
                } else {
                    Rc::new(NumberEntry::Value(Rc::new(NumberEntry::Pair(0)), new_r))
                };
                return Some((Some(value), left_value, None));
            }

            if let Some((new_right, left_value, right_value)) = Self::explode(right, &d + 1) {
                let (new_l, _) = Self::add_to_right(left, left_value);
                let value = if let Some(new_right) = new_right {
                    Rc::new(NumberEntry::Value(new_l, new_right))
                } else {
                    Rc::new(NumberEntry::Value(new_l, Rc::new(NumberEntry::Pair(0))))
                };
                return Some((Some(value), None, right_value));
            }

            None
        } else {
            None
        }
    }

    fn split(n: &Rc<NumberEntry>) -> Option<Rc<NumberEntry>> {
        match &**n {
            NumberEntry::Pair(v) => {
                if *v >= 10 {
                    let left = *v / 2; // Rounds down
                    let right = *v - left; // Total - already used, equates to rounded up

                    Some(Rc::new(NumberEntry::Value(
                        Rc::new(NumberEntry::Pair(left)),
                        Rc::new(NumberEntry::Pair(right)),
                    )))
                } else {
                    None
                }
            }
            NumberEntry::Value(l, r) => {
                if let Some(new_left) = Self::split(l) {
                    return Some(Rc::new(NumberEntry::Value(new_left, r.clone())));
                }
                if let Some(new_right) = Self::split(r) {
                    return Some(Rc::new(NumberEntry::Value(l.clone(), new_right)));
                }
                None
            }
        }
    }

    fn add(l: &Rc<NumberEntry>, r: &Rc<NumberEntry>) -> Rc<NumberEntry> {
        let mut val = Rc::new(NumberEntry::Value(l.clone(), r.clone()));
        loop {
            if let Some((Some(n), _, _)) = Self::explode(&val, 0) {
                val = n;
                continue;
            }
            if let Some(n) = Self::split(&val) {
                val = n;
                continue;
            }

            break;
        }
        val
    }

    fn sum(nums: &[Rc<NumberEntry>]) -> Rc<NumberEntry> {
        let mut ret = nums[0].clone();
        for num in nums.iter().skip(1) {
            let ret2 = Self::add(&ret, num);
            ret = ret2;
        }
        ret
    }

    fn magnitude(n: &Rc<NumberEntry>) -> u16 {
        match &**n {
            NumberEntry::Value(l, r) => 3 * Self::magnitude(l) + 2 * Self::magnitude(r),
            NumberEntry::Pair(v) => *v,
        }
    }

    fn add_to_left(n: &Rc<NumberEntry>, add_value: Option<u16>) -> (Rc<NumberEntry>, bool) {
        match add_value {
            None => (n.clone(), false),
            Some(add_value) => match &**n {
                NumberEntry::Pair(curr) => (Rc::new(NumberEntry::Pair(curr + add_value)), true),
                NumberEntry::Value(left, right) => {
                    let (new_left, did_update_left) = Self::add_to_left(left, Some(add_value));
                    if did_update_left {
                        (Rc::new(NumberEntry::Value(new_left, right.clone())), true)
                    } else {
                        (
                            Rc::new(NumberEntry::Value(
                                left.clone(),
                                Self::add_to_left(right, Some(add_value)).0,
                            )),
                            true,
                        )
                    }
                }
            },
        }
    }

    fn add_to_right(n: &Rc<NumberEntry>, add_value: Option<u16>) -> (Rc<NumberEntry>, bool) {
        match add_value {
            None => (n.clone(), false),
            Some(add_value) => match &**n {
                NumberEntry::Pair(curr) => (Rc::new(NumberEntry::Pair(curr + add_value)), true),
                NumberEntry::Value(left, right) => {
                    let (new_right, did_update_right) = Self::add_to_right(right, Some(add_value));
                    if did_update_right {
                        (Rc::new(NumberEntry::Value(left.clone(), new_right)), true)
                    } else {
                        (
                            Rc::new(NumberEntry::Value(
                                Self::add_to_right(left, Some(add_value)).0,
                                right.clone(),
                            )),
                            true,
                        )
                    }
                }
            },
        }
    }
}

fn parse(input: &PuzzleInput) -> Vec<Rc<NumberEntry>> {
    input
        .lines()
        .iter()
        .map(|s| NumberEntry::parse(s))
        .collect::<Vec<_>>()
}

fn solve_a(input: &PuzzleInput) -> u16 {
    let numbers = parse(input);
    let sum = NumberEntry::sum(&numbers);
    NumberEntry::magnitude(&sum)
}

fn solve_b(input: &PuzzleInput) -> u16 {
    let numbers = parse(input);
    let mut highest_magnitude = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            let ij_sum = NumberEntry::sum(&[numbers[i].clone(), numbers[j].clone()]);
            let ij_magnitude = NumberEntry::magnitude(&ij_sum);
            let ji_sum = NumberEntry::sum(&[numbers[j].clone(), numbers[i].clone()]);
            let ji_magnitude = NumberEntry::magnitude(&ji_sum);

            let m = std::cmp::max(ij_magnitude, ji_magnitude);
            if m > highest_magnitude {
                highest_magnitude = m;
            }
        }
    }

    highest_magnitude
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n\
                              [[[5,[2,8]],4],[5,[[9,9],0]]]\n\
                              [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n\
                              [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n\
                              [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n\
                              [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n\
                              [[[[5,4],[7,7]],8],[[8,3],8]]\n\
                              [[9,3],[[9,9],[6,[4,9]]]]\n\
                              [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n\
                              [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 4140);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 3993);
    }

    #[test]
    fn test_substring_pair_components() {
        let input = "[[1,[2,3]],4]".to_string();
        let part_1 = NumberEntry::substring_pair_component(&input, 1);
        let part_2 = NumberEntry::substring_pair_component(&input, 1 + part_1.len() + 1);
        assert_eq!(part_1, "[1,[2,3]]");
        assert_eq!(part_2, "4");
    }
}
