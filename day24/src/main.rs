use aoc_utils::PuzzleInput;
const DAY: u8 = 24;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

enum Value {
    Literal(i64),
    Variable(char),
}

impl Value {
    fn get(&self, state: &State) -> i64 {
        match self {
            Value::Literal(l) => *l,
            Value::Variable(v) => state.get(v),
        }
    }
}

enum Instruction {
    Inp(char),
    Add(char, Value),
    Mul(char, Value),
    Div(char, Value),
    Mod(char, Value),
    Eql(char, Value),
}

struct State {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl State {
    fn get(&self, c: &char) -> i64 {
        match c {
            'w' => self.w,
            'x' => self.x,
            'y' => self.y,
            'z' => self.z,
            _ => panic!("Invalid variable {}", c),
        }
    }

    fn set(&mut self, c: &char, v: i64) {
        match c {
            'w' => self.w = v,
            'x' => self.x = v,
            'y' => self.y = v,
            'z' => self.z = v,
            _ => panic!("Invalid variable {}", c),
        }
    }
}

fn parse_value(s: &str) -> Value {
    match s.parse::<_>() {
        Ok(s) => Value::Literal(s),
        Err(_) => Value::Variable(s.chars().next().unwrap()),
    }
}

fn parse_instructions(input: &PuzzleInput) -> Vec<Instruction> {
    input
        .lines()
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (instruction_mnemonic, args_str) = l.split_once(' ').unwrap();
            let args = args_str.split(' ').collect::<Vec<_>>();
            let first_arg = args[0].chars().next().unwrap();

            match instruction_mnemonic {
                "inp" => Instruction::Inp(first_arg),
                "add" => Instruction::Add(first_arg, parse_value(args[1])),
                "mul" => Instruction::Mul(first_arg, parse_value(args[1])),
                "mod" => Instruction::Mod(first_arg, parse_value(args[1])),
                "div" => Instruction::Div(first_arg, parse_value(args[1])),
                "eql" => Instruction::Eql(first_arg, parse_value(args[1])),
                _ => panic!("Unknown instruction: {}", instruction_mnemonic),
            }
        })
        .collect()
}

fn execute_instruction(instruction: &Instruction, state: &mut State, inputs: &mut Vec<i64>) {
    match instruction {
        Instruction::Inp(var) => {
            let input = inputs.pop().unwrap();
            state.set(var, input);
        }
        Instruction::Add(a, b) => {
            let a_value = state.get(a);
            let b_value = b.get(state);
            state.set(a, a_value + b_value);
        }
        Instruction::Mul(a, b) => {
            let a_value = state.get(a);
            let b_value = b.get(state);
            state.set(a, a_value * b_value);
        }
        Instruction::Div(a, b) => {
            let a_value = state.get(a);
            let b_value = b.get(state);
            state.set(a, a_value / b_value);
        }
        Instruction::Mod(a, b) => {
            let a_value = state.get(a);
            let b_value = b.get(state);
            state.set(a, a_value % b_value);
        }
        Instruction::Eql(a, b) => {
            let a_value = state.get(a);
            let b_value = b.get(state);
            state.set(a, if a_value == b_value { 1 } else { 0 });
        }
    }
}

fn run_program(instructions: &[Instruction], input_values: Vec<i64>) -> i64 {
    let mut state = State {
        w: 0,
        x: 0,
        y: 0,
        z: 0,
    };
    let mut remaining_inputs = input_values;
    remaining_inputs.reverse();

    for instruction in instructions.iter() {
        execute_instruction(instruction, &mut state, &mut remaining_inputs);
    }

    state.z
}

fn verify(input: &PuzzleInput, model_number: i64) -> i64 {
    // Brute forcing this is not really possible.
    // Instead analyze the program on paper and find corresponding inputs for your puzzle input.
    let instructions = parse_instructions(input);

    // Verify the model number is correct using the program
    let model_number_string = format!("{}", model_number);
    let input = model_number_string
        .chars()
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .collect();
    println!("input: {:?}", input);

    if run_program(&instructions, input) == 0 {
        model_number
    } else {
        panic!("model number is not valid");
    }
}

fn solve_a(input: &PuzzleInput) -> i64 {
    verify(input, 99196997985942)
}

fn solve_b(input: &PuzzleInput) -> i64 {
    verify(input, 84191521311611)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_basic_program() {
        let instructions = "inp w\n\
                            add z w\n\
                            mod z 2\n\
                            div w 2\n\
                            add y w\n\
                            mod y 2\n\
                            div w 2\n\
                            add x w\n\
                            mod x 2\n\
                            div w 2\n\
                            mod w 2";

        let program = parse_instructions(&PuzzleInput::new(instructions));
        let input_values = vec![5];
        let result = run_program(&program, input_values);
        assert_eq!(result, 1);
    }
}
