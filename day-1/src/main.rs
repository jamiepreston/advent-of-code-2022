use std::fs;
use std::vec::Vec;
use std::cmp::Ordering;

fn main() {
    let file_path: &str = "puzzle-input.txt";
    // let file_path: &str = "test-input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut vec: Vec<String> = Vec::new();
    let mut elves: Vec<String> = Vec::new();
    let mut biggest: u32 = 0;
    let mut second: u32 = 0;
    let mut third: u32 = 0;

    let split = contents.split("\n");
    for s in split {
        // println!("-{}", s);
        match s.cmp("") {
            Ordering::Equal => {
                let mut total: u32 = 0;
                for s in &vec {
                    let parsedNum: u32 = match s.trim().parse() {
                        Ok(num) => num,
                        Err(_) => 0,
                    };
                    total = &total + parsedNum;
                }

                match total.cmp(&biggest) {
                    Ordering::Greater => {
                        third = second;
                        second = biggest;
                        biggest = total;
                    },
                    _ => {
                        match total.cmp(&second) {
                            Ordering::Greater => {
                                second = total;
                            },
                            _ => {
                                match total.cmp(&third) {
                                    Ordering::Greater => {
                                        third = total;
                                    },
                                    _ => ()
                                };
                            },
                        };
                    },
                };

                let parsedTotal: String = total.to_string().to_owned();
                elves.push(total.to_string().to_owned());
                vec.clear();
            },
            _ => {
                vec.push(s.to_string().to_owned());
            }
        }
    }
    println!("biggest: {}", biggest);
    println!("second: {}", second);
    println!("third: {}", third);
    println!("total: {}", biggest + second + third);

}
