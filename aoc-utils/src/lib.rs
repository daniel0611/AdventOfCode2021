use std::fs;
use std::path::Path;

pub struct PuzzleInput {
    pub raw_input: String,
}

impl PuzzleInput {
    fn get_input<T: AsRef<Path>>(day: u8, path: T) -> PuzzleInput {
        // When running in tests, the working directory is inside the package, but
        // when running the actual puzzle, it's in the workspace root.
        let outside_path_str = format!("day{}/{}", day, path.as_ref().display());
        let outside_path = Path::new(&outside_path_str);

        if path.as_ref().exists() {
            let content = fs::read_to_string(path.as_ref()).expect("Unable to read file");
            PuzzleInput { raw_input: content }
        } else if outside_path.exists() {
            let content = fs::read_to_string(outside_path).expect("Unable to read file");
            PuzzleInput { raw_input: content }
        } else {
            let cwd = std::env::current_dir().unwrap();
            panic!(
                "Puzzle input at {}/{} does not exist",
                cwd.display(),
                path.as_ref().display()
            );
        }
    }

    /// Reads the puzzle input for puzzle A for the given day from inputs/<day_number>a.txt
    pub fn get_input_a(day_number: u8) -> PuzzleInput {
        let path = format!("inputs/{}a.txt", day_number);
        PuzzleInput::get_input(day_number, &path)
    }

    /// Reads the puzzle input for puzzle B for the given day from inputs/<day_number>a.txt
    pub fn get_input_b(day_number: u8) -> PuzzleInput {
        let path = format!("inputs/{}b.txt", day_number);
        PuzzleInput::get_input(day_number, &path)
    }

    pub fn lines(&self) -> Vec<String> {
        self.raw_input.lines().map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    // Should be able to get input "Hello, this is a test" from day 0
    #[test]
    fn test_get_input_0a_success() {
        let input = super::PuzzleInput::get_input_a(0);
        assert_eq!(input.raw_input, "Hello, this is a test\n");
    }

    // File for puzzle b on day 0 doesn't exist, so should panic
    #[test]
    #[should_panic]
    fn test_get_input_0b_fail() {
        super::PuzzleInput::get_input_b(0);
    }
}
