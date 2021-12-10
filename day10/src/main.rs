use aoc_utils::PuzzleInput;
const DAY: u8 = 10;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

#[derive(PartialEq, Eq, Debug)]
enum CharacterType {
    Parenthesis,
    SquareBracket,
    CurlyBracket,
    AngleBracket,
}

impl CharacterType {
    fn from_char(c: char) -> CharacterType {
        match c {
            '(' => CharacterType::Parenthesis,
            ')' => CharacterType::Parenthesis,
            '[' => CharacterType::SquareBracket,
            ']' => CharacterType::SquareBracket,
            '{' => CharacterType::CurlyBracket,
            '}' => CharacterType::CurlyBracket,
            '<' => CharacterType::AngleBracket,
            '>' => CharacterType::AngleBracket,
            _ => panic!("Invalid character"),
        }
    }

    fn get_corrupted_points(&self) -> usize {
        match self {
            CharacterType::Parenthesis => 3,
            CharacterType::SquareBracket => 57,
            CharacterType::CurlyBracket => 1197,
            CharacterType::AngleBracket => 25137,
        }
    }

    fn get_completion_points(&self) -> usize {
        match self {
            CharacterType::Parenthesis => 1,
            CharacterType::SquareBracket => 2,
            CharacterType::CurlyBracket => 3,
            CharacterType::AngleBracket => 4,
        }
    }
}

fn is_character_opening(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '<')
}

fn solve_a(input: &PuzzleInput) -> usize {
    let lines = input.lines();
    let mut points = 0;

    for line in lines.iter() {
        let mut characters = vec![];
        for c in line.chars() {
            let t = CharacterType::from_char(c);
            if is_character_opening(c) {
                characters.push(t);
            } else {
                let last = characters.pop().unwrap();
                if last != t {
                    // Wrong closing character
                    points += t.get_corrupted_points();
                }
            }
        }

        // incomplete lines ignored
    }

    points
}

fn solve_b(input: &PuzzleInput) -> usize {
    let lines = input.lines();
    let mut points = vec![];

    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }

        let mut characters = vec![];
        let mut invalid = false;
        for c in line.chars() {
            let t = CharacterType::from_char(c);
            if is_character_opening(c) {
                characters.push(t);
            } else {
                let last = characters.pop().unwrap();
                if last != t {
                    // Wrong closing character => corrupted, ignore
                    invalid = true;
                }
            }
        }
        if invalid {
            continue;
        }

        // incomplete
        characters.reverse(); // We need the closing characters so we need to reverse this

        let mut line_points = 0;
        for c in characters.iter() {
            line_points = line_points * 5 + c.get_completion_points();
        }
        points.push(line_points);
    }

    points.sort_unstable();
    assert_eq!(points.len() % 2, 1);
    if points.len() == 1 {
        points[0]
    } else {
        points[points.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>\n\
        [(()[<>])]({[<{<<[]>>(\n\
        {([(<{}[<>[]}>{[]{[(<()>\n\
        (((({<>}<{<{<>}{[]{[]{}\n\
        [[<[([]))<([[{}[[()]]]\n\
        [{[{({}]{}}([{[{{{}}([]\n\
        {<[[]]>}<{[{[{[]{()[[[]\n\
        [<(<(<(<{}))><([]([]()\n\
        <{([([[(<>()){}]>(<<{{\n\
        <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 26397);
    }

    // #[test]
    // fn test_solve_b() {
    //     assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 0);
    // }

    #[test]
    fn test_solve_b_easy_1() {
        let input = "[({(<(())[]>[[{[]{<()<>>";
        assert_eq!(solve_b(&PuzzleInput::new(input)), 288957);
    }

    #[test]
    fn test_solve_b_easy_2() {
        let input = "<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(solve_b(&PuzzleInput::new(input)), 294);
    }

    #[test]
    fn test_solve_b_easy_3() {
        let input = "[(()[<>])]({[<{<<[]>>(\n(((({<>}<{<{<>}{[]{[]{}\n{<[[]]>}<{[{[{[]{()[[[]";
        assert_eq!(solve_b(&PuzzleInput::new(input)), 995444);
    }

    #[test]
    fn test_solve_b_easy_full() {
        let input = "[(()[<>])]({[<{<<[]>>(\n(((({<>}<{<{<>}{[]{[]{}\n{<[[]]>}<{[{[{[]{()[[[]";
        assert_eq!(solve_b(&PuzzleInput::new(input)), 995444);
    }
}
