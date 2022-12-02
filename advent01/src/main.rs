use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

fn main() {
    let file = File::open("advent01/input").expect("no such file");
    let buf = BufReader::new(file);
    let elfs = get_elfs(buf);
    let mut max_1 = 0;
    let mut max_2 = 0;
    let mut max_3 = 0;
    for (_, elf_count) in elfs {
        if elf_count > max_1 {
            max_3 = max_2;
            max_2 = max_1;
            max_1 = elf_count;
        } else if elf_count > max_2 {
            max_3 = max_2;
            max_2 = elf_count;
        } else if elf_count > max_3 {
            max_3 = elf_count;
        }
    }
    println!("max 1 is {}", max_1);
    println!("max 2 is {}", max_2);
    println!("max 3 is {}", max_3);
    println!("max is {}", max_1 + max_2 + max_3);
}

fn get_elfs(buf: BufReader<File>) -> HashMap<usize, u64> {
    let mut elfs = HashMap::new();
    let mut current_elf: usize = 1;
    let mut current_count: u64 = 0;
    for line in buf.lines() {
        match line {
            Ok(data) => {
                if data.is_empty() {
                    elfs.insert(current_elf, current_count);
                    println!("{} has {}", current_elf, current_count);
                    current_elf += 1;
                    current_count = 0;

                } else {
                    current_count += data.parse::<u64>().unwrap();
                }

            },
            Err(e) => println!("Finished reading {}", e),
        }
    }
    elfs
}