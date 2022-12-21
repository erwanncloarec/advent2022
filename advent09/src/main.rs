use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    pub fn from_str(str: &str) -> Self {
        match str {
            "R" => Direction::Right,
            "U" => Direction::Up,
            "L" => Direction::Left,
            "D" => Direction::Down,
            _ => panic!("unknown direction"),
        }
    }
}

fn main() {
    let file = File::open("advent09/input").expect("no such file");
    let buf = BufReader::new(file);
    let mut h_pos = (0, 0);
    let mut t_pos_arr = vec![(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0)];
    let mut visited_pos = vec![(0,0)];
    for line in buf.lines() {
        println!("h({},{}) {}", h_pos.0, h_pos.1, print_t_pos(&t_pos_arr));
        let s_line = line.unwrap();
        let mut split_line = s_line.split(' ');
        let direction = Direction::from_str(split_line.next().unwrap());
        let distance = split_line.next().unwrap().parse::<i16>().unwrap();
        println!("Moving {} in {:?}", distance, direction);
        for _ in 0..distance {
            h_pos = get_new_h_pos(h_pos, direction);
            let mut new_t_pos_arr = Vec::with_capacity(9);
            for i in 0..t_pos_arr.len() {
                let t_pos = t_pos_arr.get(i).unwrap();
                if i==0 {
                    new_t_pos_arr.push(get_new_t_pos(t_pos, &h_pos));
                } else {
                    let t_pos_prec = t_pos_arr.get(i-1).unwrap();
                    new_t_pos_arr.push(get_new_t_pos(t_pos, t_pos_prec));
                }
                if i == 8 && !visited_pos.contains(t_pos) {
                    visited_pos.push((t_pos.0, t_pos.1));
                }
            }
            t_pos_arr = new_t_pos_arr;
        }
    }
    println!("visited {} positions", visited_pos.len());
}

fn print_t_pos(t_pos: &[(i16, i16)]) -> String {
    let mut res = "".to_string();
    for (i, t_pos_i) in t_pos.iter().enumerate() {
        res = format!("{} t{}({},{})", res, i+1, t_pos_i.0, t_pos_i.1);
    }
    res
}

fn get_new_t_pos(t_pos: &(i16, i16), h_pos: &(i16, i16)) -> (i16, i16) {
    let mut res = (t_pos.0, t_pos.1);
    let ecart_x = h_pos.0 - t_pos.0;
    let ecart_y = h_pos.1 - t_pos.1;
    assert!(ecart_x <= 2);
    assert!(ecart_x >= -2);
    assert!(ecart_y <= 2);
    assert!(ecart_y >= -2);
    if ecart_x.abs() > 0 && ecart_y.abs() > 0 && (ecart_x.abs() == 2 || ecart_y.abs() == 2) {
        res.0 += ecart_x.signum();
        res.1 += ecart_y.signum();
        return res;
    }
    if ecart_x.abs() == 2 {
        res.0 += ecart_x.signum()
    }
    if ecart_y.abs() == 2 {
        res.1 += ecart_y.signum()
    }
    res
}

fn get_new_h_pos(original_pos: (i16, i16), direction: Direction) -> (i16, i16) {
    match direction {
        Direction::Right => (original_pos.0 + 1, original_pos.1),
        Direction::Up => (original_pos.0, original_pos.1 + 1),
        Direction::Left => (original_pos.0 - 1, original_pos.1),
        Direction::Down => (original_pos.0, original_pos.1 - 1),
    }
}
