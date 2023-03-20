use std::fs;
use std::vec::Vec;

fn main() {
    let file_path: &str = "puzzle-input.txt";
    // let file_path: &str = "test-input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut score: u32 = 0;

    let split = contents.split("\n");
    for s in split {
        match s {
            "" => (),
            _ => {
                let char_vec: Vec<char> = s.chars().collect();

                let this_score: u32 = match char_vec[2] {
                    'X' => 1,
                    'Y' => 2,
                    'Z' => 3,
                    _ => 0,
                };

                let result: u32 = match (char_vec[0], char_vec[2]) {
                    ('A', 'X') => 3,
                    ('B', 'Y') => 3,
                    ('C', 'Z') => 3,
                    ('A', 'Y') => 6,
                    ('B', 'Z') => 6,
                    ('C', 'X') => 6,
                    _ => 0,
                };
                score = &score + (this_score + result);
            }
        };

    }
    println!("score: {}", score);

}
