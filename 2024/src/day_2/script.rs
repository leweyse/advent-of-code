use std::ffi::OsString;
use std::fs::File;
use std::io::Read;

const SHOULD_DECREASE: i32 = 0;
const SHOULD_INCREASE: i32 = 1;

fn validate_levels(levels: &Vec<i32>) -> bool {
    // should_increase_or_decrease
    let mut siod = -1;
    let mut is_safe = 0;

    for idx in 0..levels.len() {
        let curr_level = levels[idx];
        let next_level = match levels.get(idx + 1) {
            Some(level) => level,
            None => break, // check
        };

        let diff = next_level - curr_level;

        if diff == 0 || diff.abs() > 3 {
            break;
        }

        if siod == -1 {
            if diff < 0 {
                siod = SHOULD_DECREASE;
            } else {
                siod = SHOULD_INCREASE;
            }
        }

        let valid_op = match siod {
            SHOULD_INCREASE => &curr_level < next_level,
            SHOULD_DECREASE => &curr_level > next_level,
            _ => panic!("Error parsing levels: {:?}", levels),
        };

        if !valid_op {
            break;
        }

        is_safe += 1;
    }

    is_safe == levels.len() - 1
}

pub fn part_1() {
    let input_file = OsString::from("./src/day_2/input.txt");

    let mut file = match File::open(input_file) {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {}", e),
    };

    let mut reports = String::new();
    match file.read_to_string(&mut reports) {
        Ok(_) => {}
        Err(e) => panic!("Error reading file: {}", e),
    };

    let mut save_count = 0;

    for report in reports.lines() {
        let r = report.trim();

        if r.is_empty() {
            continue;
        }

        let levels: Vec<i32> = r.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        let is_safe = validate_levels(&levels);

        if is_safe {
            save_count += 1;
        }
    }

    println!("{}", save_count);
}

pub fn part_2() {
    let input_file = OsString::from("./src/day_2/input.txt");

    let mut file = match File::open(input_file) {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {}", e),
    };

    let mut reports = String::new();
    match file.read_to_string(&mut reports) {
        Ok(_) => {}
        Err(e) => panic!("Error reading file: {}", e),
    };

    let mut save_count = 0;

    for report in reports.lines() {
        let r = report.trim();

        if r.is_empty() {
            continue;
        }

        let levels: Vec<i32> = r.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        let is_safe = validate_levels(&levels);

        if is_safe {
            save_count += 1;
        } else {
            let levels_copy = Vec::from(levels.clone());

            for idx in 0..levels.len() {
                let start_slice = &levels_copy[0..idx];
                let end_slice = &levels_copy[idx + 1..];

                let mut copy = Vec::from(start_slice);
                let mut end_vec = Vec::from(end_slice);
                copy.append(&mut end_vec);

                let is_valid = validate_levels(&copy);

                if is_valid {
                    save_count += 1;
                    break;
                }
            }
        }
    }

    println!("{}", save_count);
}
