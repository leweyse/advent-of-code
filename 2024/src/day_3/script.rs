use std::ffi::OsString;
use std::fs::File;
use std::io::Read;

use regex::bytes::Regex;

const INPUT_FILE_PATH: &str = "./src/day_3/input.txt";

// I hate regex, and you know what?
// Rust did not make it easy.

pub fn part_1() {
    let input_file = OsString::from(INPUT_FILE_PATH);

    let mut file = match File::open(input_file) {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {}", e),
    };

    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Ok(_) => {}
        Err(e) => panic!("Error reading file: {}", e),
    };

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let instructions = re.captures_iter(file_content.as_bytes())
        .map(|capture| {
            if let Some(first) = capture.get(1) {
                let f = match atoi::atoi::<usize>(first.as_bytes()) {
                    Some(r) => r,
                    None => panic!("Could not read")
                };

                if let Some(second) = capture.get(2) {
                    let s = match atoi::atoi::<usize>(second.as_bytes()) {
                        Some(r) => r,
                        None => panic!("Could not read")
                    };

                    return f * s
                }
            }

            return 0;
        });

    println!("{:?}", instructions.sum::<usize>());
}

pub fn part_2() {
    let input_file = OsString::from(INPUT_FILE_PATH);

    let mut file = match File::open(input_file) {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {}", e),
    };

    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Ok(_) => {}
        Err(e) => panic!("Error reading file: {}", e),
    };

    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();

    let mut enabled = true;

    let instructions = re.captures_iter(file_content.as_bytes())
        .filter(|instruction| {
            if let Some(start) = instruction.get(0) {
                if start.as_bytes() == b"do()" {
                    enabled = true;
                    return false;
                } else if start.as_bytes() == b"don't()" {
                    enabled = false;
                }
            }

            return enabled;
        })
        .map(|instruction| {
            if let Some(first) = instruction.get(2) {
                let f = match atoi::atoi::<usize>(first.as_bytes()) {
                    Some(r) => r,
                    None => panic!("Could not read")
                };


                if let Some(second) = instruction.get(3) {
                    let s = match atoi::atoi::<usize>(second.as_bytes()) {
                        Some(r) => r,
                        None => panic!("Could not read")
                    };

                    return f * s
                }
            }

            return 0;
        });

    println!("{:?}", instructions.sum::<usize>());
}
