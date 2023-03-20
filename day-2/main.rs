use std::fs;
use std::vec::Vec;

// PART 2 SOLUTION

fn get_score_for_move(m: char) -> u32 {
    return match m {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => 0,
    };
}
fn get_score_for_result(r: char) -> u32 {
    return match r {
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    };
}

fn find_correct_move(m: char, outcome: char) -> char {
    let winning_move: char = match m {
        'A' => 'B',
        'B' => 'C',
        _ => 'A',
    };
    let losing_move: char = match m {
        'A' => 'C',
        'B' => 'A',
        _ => 'B',
    };
    return match outcome {
        'X' => losing_move,
        'Z' => winning_move,
        _ => m,
    };
}

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

                let correct_move = find_correct_move(char_vec[0], char_vec[2]);
                let score_for_move = get_score_for_move(correct_move);
                let score_for_result = get_score_for_result(char_vec[2]);
                score = &score + (score_for_move + score_for_result);
            }
        };

    }
    println!("score: {}", score);

}
