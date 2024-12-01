use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;

fn parse_input_file(input_file: OsString, sort: bool) -> Vec<Vec<i32>> {
    let mut file = match File::open(input_file) {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {}", e),
    };

    let mut lines = String::new();
    match file.read_to_string(&mut lines) {
        Ok(_) => {}
        Err(e) => panic!("Error reading file: {}", e),
    };

    let mut lists: Vec<Vec<i32>> = Vec::new();

    for line in lines.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let cols: Vec<&str> = line.split("   ").map(|x| x.trim()).collect();

        for idx in 0..cols.len() {
            match cols.get(idx) {
                Some(_) => {
                    let num = match cols[idx].parse::<i32>() {
                        Ok(num) => num,
                        Err(_) => panic!("Error parsing line: {}", line),
                    };

                    match lists.get(idx) {
                        Some(_) => lists[idx].push(num),
                        None => lists.push(vec![num]),
                    }
                }
                None => panic!("Error parsing line: {}", line),
            }
        }
    }

    if sort {
        for list in &mut lists {
            list.sort();
        }
    }

    lists
}

pub fn part_1() {
    let input_file = OsString::from("./src/day_1/part_1.txt");

    let lists = parse_input_file(input_file, true);

    let mut sum = 0;

    // assumption: the lists are the same length
    for idx in 0..lists[0].len() {
        // only two lists are being compared
        // so the following is not the simplest
        // way to do it... but it works
        let mut items = Vec::new();

        for list in &lists {
            items.push(list[idx]);
        }

        let highest = items.iter().max().unwrap();
        let lowest = items.iter().min().unwrap();

        sum += highest - lowest;
    }

    println!("{}", sum);
}

pub fn part_2() {
    let input_file = OsString::from("./src/day_1/part_2.txt");

    let lists = parse_input_file(input_file, false); // len = 2

    let mut cache = HashMap::new();

    let mut sum = 0;

    for item in &lists[0] {
        if let Some(cached) = cache.get(item) {
            sum += *cached;
            continue;
        }

        let mut score = 0;

        for score_item in &lists[1] {
            if item == score_item {
                score += 1;
            }
        }

        let item_score = score * item;

        cache.insert(item, item_score);
        sum += item_score;
    }

    println!("{}", sum);
}
