use std::{fs::File, io::{BufReader, Lines, BufRead}, fmt::Display};

fn main() {
    let file = File::open("advent04/input").expect("no such file");
    let buf = BufReader::new(file);
    let lines = buf.lines();
    println!("full overlaps: {}", count_full_overlaps(lines));
    let file = File::open("advent04/input").expect("no such file");
    let buf = BufReader::new(file);
    let lines = buf.lines();
    println!("partial overlaps: {}", count_partial_overlaps(lines))
}


struct Assignement {
    min: u16,
    max: u16,
}

impl Assignement {
    fn from(id: &str) -> Self {
        let mut split = id.split('-');
        let min: u16 = split.next().unwrap().parse().unwrap();
        let max: u16 = split.next().unwrap().parse().unwrap();
        assert!(min <= max);
        Assignement { min, max }
    }

    fn fully_overlaps(&self, other: &Self) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn partial_overlaps(&self, other: &Self) -> bool {
        (self.min <= other.max && self.min >= other.min) || (self.max >= other.min && self.max <= other.max) || self.min <= other.min && self.max >= other.max
    }
}

impl Display for Assignement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "min:{} max:{}", self.min, self.max)
    }
}

fn count_full_overlaps(lines: Lines<BufReader<File>>) -> u16 {
    let mut overlap_count = 0;
    for line in lines {
        let s_line = line.unwrap();
        let mut ids = s_line.split(',');
        let id_1 = Assignement::from(ids.next().unwrap());
        let id_2 = Assignement::from(ids.next().unwrap());
        if id_1.fully_overlaps(&id_2) || id_2.fully_overlaps(&id_1) {
            overlap_count += 1;
        }
    }
    overlap_count
}

fn count_partial_overlaps(lines: Lines<BufReader<File>>) -> u16 {
    let mut overlap_count = 0;
    for line in lines {
        let s_line = line.unwrap();
        let mut ids = s_line.split(',');
        let id_1 = Assignement::from(ids.next().unwrap());
        let id_2 = Assignement::from(ids.next().unwrap());
        if id_1.partial_overlaps(&id_2) {
            overlap_count += 1;
        } else {
            println!("no partial overlap: {} VS {}", id_1, id_2);
        }
    }
    overlap_count
}