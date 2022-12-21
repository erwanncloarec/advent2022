use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx,
}

impl Instruction {
    pub fn from_str(str: &str) -> Self {
        match str {
            "addx" => Instruction::Addx,
            "noop" => Instruction::Noop,
            _ => panic!("unknown instruction"),
        }
    }
}

fn main() {
    first_part();
    second_part();
}


fn first_part() {
    let file = File::open("advent10/input").expect("no such file");
    let buf = BufReader::new(file);
    let mut x: i32 = 1;
    let mut cycle = 0;
    let mut next_cycle = 19;
    let mut signals_strength = 0;
    for line in buf.lines() {
        let s_line = line.unwrap();
        let mut split_line = s_line.split(' ');
        let instruction = Instruction::from_str(split_line.next().unwrap());

        match instruction {
            Instruction::Noop => {
                cycle += 1;
                if cycle - next_cycle == 0 {
                    let signal_strength = x * (next_cycle + 1);
                    println!(
                        "Signal strength on {}th cycle : {}",
                        next_cycle + 1,
                        signal_strength
                    );
                    signals_strength += signal_strength;
                    next_cycle += 40;
                }
            },
            Instruction::Addx => {
                println!("Addx {}", cycle);
                let value = split_line.next().unwrap().parse::<i32>().unwrap();
                cycle += 1;
                if cycle - next_cycle == 0 {
                    let signal_strength = x * (next_cycle + 1);
                    println!(
                        "Signal strength on {}th cycle : {}",
                        next_cycle + 1,
                        signal_strength
                    );
                    signals_strength += signal_strength;
                    next_cycle += 40;
                }
                cycle += 1;
                x += value;
                if cycle - next_cycle == 0 {
                    let signal_strength = x * (next_cycle + 1);
                    println!(
                        "Signal strength on {}th cycle : {}",
                        next_cycle + 1,
                        signal_strength
                    );
                    signals_strength += signal_strength;
                    next_cycle += 40;
                }
            }
        }
    }
    println!("Signals strength {}", signals_strength);
}


fn second_part() {
    let file = File::open("advent10/input").expect("no such file");
    let buf = BufReader::new(file);
    let mut x: i32 = 1;
    let mut cycle = 0;
    let mut next_cycle = 39;
    let mut crt: Vec<Vec<char>> = Vec::with_capacity(6);
    let mut pixels: Vec<char> = Vec::with_capacity(40);
    pixels.push('.');
    for line in buf.lines() {
        let s_line = line.unwrap();
        let mut split_line = s_line.split(' ');
        let instruction = Instruction::from_str(split_line.next().unwrap());

        match instruction {
            Instruction::Noop => {
                pixels.push(draw_pixel(cycle % 40, x));
                cycle += 1;
                if cycle - next_cycle == 0 {
                    println!("Line finished");
                    print_pixels(&pixels);
                    crt.push(pixels);
                    pixels = Vec::with_capacity(40);
                    next_cycle += 40;
                }
            },
            Instruction::Addx => {
                let value = split_line.next().unwrap().parse::<i32>().unwrap();
                pixels.push(draw_pixel(cycle % 40, x));
                cycle += 1;
                if cycle - next_cycle == 0 {
                    println!("Line finished");
                    print_pixels(&pixels);
                    crt.push(pixels);
                    pixels = Vec::with_capacity(40);
                    next_cycle += 40;
                }
                pixels.push(draw_pixel(cycle % 40, x));
                cycle += 1;
                x += value;
                if cycle - next_cycle == 0 {
                    println!("Line finished");
                    print_pixels(&pixels);
                    crt.push(pixels);
                    pixels = Vec::with_capacity(40);
                    next_cycle += 40;
                }
            }
        }
    }
    println!();
    println!("-----------");
    println!();
    for pixels in crt {
        print_pixels(&pixels);
    }
}

fn print_pixels(pixels: &[char]) {
    for pixel in pixels {
        print!("{}", pixel);
    }
    println!();
}

fn draw_pixel(cycle: i32, x: i32) -> char {
    if x-1 == cycle || x == cycle || x+1 == cycle {
        '#'
    } else {
        '.'
    }
}