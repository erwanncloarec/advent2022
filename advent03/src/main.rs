use std::{fs::File, io::{BufReader, Lines, BufRead}};

fn main() {
    let file = File::open("advent03/input").expect("no such file");
    let buf = BufReader::new(file);
    println!("sum : {}", sum_item_types_by_groups(buf.lines()))
}

fn sum_item_types(lines: Lines<BufReader<File>>) -> u64 {
    let mut sum: u64 = 0;
    for line in lines {
        let s_line = line.unwrap();
        let (front_compartment, back_compartment) = s_line.split_at(s_line.len() / 2);
        sum += find_item(front_compartment, back_compartment) as u64;
    }
    sum
}

fn sum_item_types_by_groups(lines: Lines<BufReader<File>>) -> u64 {
    let mut sum: u64 = 0;
    let mut line_group: Vec<String> = Vec::with_capacity(3);
    for (idx, line) in lines.enumerate() {
        let s_line = line.unwrap();
        println!("reading line {}", s_line.clone());
        line_group.push(s_line);
        if idx % 3 == 2 {
            let line_1 = line_group[0].clone();
            let line_2 = line_group[1].clone();
            let line_3 = line_group[2].clone();
            println!("line 1 {}", line_1);
            println!("line 2 {}", line_2);
            println!("line 3 {}", line_3);
            sum += find_item_double_line(&line_1, &line_2, &line_3) as u64;
            line_group.clear();
        }
    }
    sum
}

fn find_item_double_line(source: &str, test_1: &str, test_2: &str) -> u32 {
    for cha in source.chars() {
        if test_1.contains(cha) && test_2.contains(cha) {
           return get_char_value(cha)
        }
    }
    panic!()
}

fn find_item(source: &str, target: &str) -> u32 {
    for cha in source.chars() {
        if target.contains(cha) {
           return get_char_value(cha) 
        }
    }
    panic!()
}

fn get_char_value(cha: char) -> u32 {
    if ('a'..='z').contains(&cha) {
        return cha as u32 - 96
    } else if ('A'..='Z').contains(&cha) {
        return cha as u32 - 38
    }
    panic!()
}