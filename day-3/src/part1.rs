use std::fs;
use std::vec::Vec;



fn find_overlap(v1: Vec<&char>, v2: Vec<&char>) -> char {
    let mut overlaps = Vec::new();
    for c in v1 {
        let is_match:bool = v2.contains(&c);
        match is_match {
            true => overlaps.push(c),
            false => (),
        }
    }
    return *overlaps[0];
}

fn find_char_value(c: char) -> usize {
    let alphabet = vec!['_','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    return alphabet.iter().position(|&r| r == c).unwrap();
}

fn main() {
    let file_path: &str = "puzzle-input.txt";
    // let file_path: &str = "test-input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total: usize = 0;
    let split = contents.split("\n");
    for s in split {
        match s {
            "" => (),
            _ => {
                let middle_point = s.chars().count() / 2;
                let chars: Vec<char> = s.chars().collect();
                let (left, right) = chars.split_at(middle_point);
                let lv:Vec<_> = Vec::from_iter(left);
                let rv:Vec<_> = Vec::from_iter(right);
                let overlap: char = find_overlap( lv,rv);
                let char_value: usize = find_char_value(overlap);
                total = &total + char_value;
            }
        }
    }
    println!("total: {}", total);

}
