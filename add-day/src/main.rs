use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

// This is a utility that asks the user for a input number
// and then creates a new crate for the advent of code challange of that day.

fn main() {
    let day = get_day();
    let day_dir = create_day_dir(day);
    create_cargo_toml(day, &day_dir);
    create_src(day, &day_dir);
    create_inputs(day, &day_dir);
}

fn get_day() -> u8 {
    print!("Please enter the day you want to create a crate for: ");
    io::stdout().flush().unwrap();

    let mut day = String::new();
    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line");
    day.trim().parse::<u8>().expect("Please enter a number!")
}

fn create_day_dir<'a>(day: u8) -> PathBuf {
    let day_directory_str = format!("day{}", day);
    let day_directory = PathBuf::from(&day_directory_str);
    fs::create_dir(&day_directory).expect("Could not create day directory");

    day_directory
}

fn create_cargo_toml(day: u8, day_dir: &PathBuf) {
    let cargo_toml_path = day_dir.join("Cargo.toml");
    let cargo_toml_str = format!(
        r#"[package]
name = "day{day}"
version = "0.1.0"
edition = "2021"

[dependencies]
aoc-utils = {{ path = "../aoc-utils" }}
"#,
        day = day
    );
    fs::write(&cargo_toml_path, cargo_toml_str).expect("Could not write Cargo.toml");
}

fn create_src(day: u8, day_dir: &PathBuf) {
    let src_dir = &day_dir.join("src");
    fs::create_dir(src_dir).expect("Could not create src directory");

    let main_rs_path = src_dir.join("main.rs");
    let main_rs_str = format!(
        r#"use aoc_utils::PuzzleInput;
const DAY: u8 = {day};

fn main() {{
    println!("A: {{}}", solve_a());
    println!("B: {{}}", solve_b());
}}

fn solve_a() -> usize {{
    let input = PuzzleInput::get_input_a(DAY);
    input.lines().len()
}}

fn solve_b() -> usize {{
    let input = PuzzleInput::get_input_a(DAY);
    input.lines().len()
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_no_panic() {{
        solve_a();
        solve_b();
    }}
}}
"#,
        day = day
    );
    fs::write(&main_rs_path, main_rs_str).expect("Could not write main.rs");
}

fn create_inputs(day: u8, day_dir: &PathBuf) {
    let inputs_directory = &day_dir.join("inputs");
    fs::create_dir(inputs_directory).expect("Could not create inputs directory");

    for challange in ['a', 'b'] {
        let name = format!("{}{}.txt", day, challange);
        let input_path = inputs_directory.join(name);
        fs::File::create(input_path).expect("Could not create input file");
    }
}
