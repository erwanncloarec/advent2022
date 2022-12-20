use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Lines}, fmt::format,
};

struct State {
    state: HashMap<u16, VecDeque<char>>,
}

impl State {
    fn new() -> Self {
        Self {
            state: get_initial_state(),
        }
    }

    fn execute_moves(&mut self, lines: Lines<BufReader<File>>) {
        println!("starting");
        self.print_tops();
        let s_lines = lines.skip(10);
        for line in s_lines {
            let s_line = line.unwrap();
            let mut split_line = s_line.split(' ');
            let crate_count = split_line.nth(1).unwrap().parse::<u16>().unwrap();
            let origin = split_line.nth(1).unwrap().parse::<u16>().unwrap();
            let dest = split_line.nth(1).unwrap().parse::<u16>().unwrap();
            println!(
                "moving {} crate(s) from {} to {}",
                crate_count, origin, dest
            );

            let mut temp = VecDeque::new();

            let orig_crate = self.state.get_mut(&origin).unwrap();
            for _ in 0..crate_count {
                let crate_to_move = orig_crate.pop_front().unwrap();
                //dest_crate.push_front(crate_to_move);
                temp.push_front(crate_to_move);
            }

            let dest_crate = self.state.get_mut(&dest).unwrap();
            for tem in temp {
                dest_crate.push_front(tem);
            }

            self.print_tops();
        }

        self.print_tops();
    }

    fn print_tops(&self) {
        //for i in 1..10 {
        print!("| ");
        for i in 1..10 {
            let state = self.state.get(&i).unwrap();
            print!(
                "{}: \"{}\" ({}) | ",
                i,
                //state.front().unwrap_or(&'_'),
                print_vecdeque(state),
                state.len()
            )
        }
        println!()
    }

    
}

fn print_vecdeque(state: &VecDeque<char>) -> String {
    if state.is_empty() {
        return "_".to_string();
    }
    let mut res = "".to_string();
    for cha in state {
        res = format!("{}{}", res, cha);
    }
    res
} 

fn main() {
    let file = File::open("advent05/input").expect("no such file");
    let buf = BufReader::new(file);
    let lines = buf.lines();
    let mut state = State::new();
    state.execute_moves(lines);
    println!("---------------------------");
}

fn get_initial_state() -> HashMap<u16, VecDeque<char>> {
    /*
        [B]             [B] [S]
        [M]             [P] [L] [B] [J]
        [D]     [R]     [V] [D] [Q] [D]
        [T] [R] [Z]     [H] [H] [G] [C]
        [P] [W] [J] [B] [J] [F] [J] [S]
    [N] [S] [Z] [V] [M] [N] [Z] [F] [M]
    [W] [Z] [H] [D] [H] [G] [Q] [S] [W]
    [B] [L] [Q] [W] [S] [L] [J] [W] [Z]
     1   2   3   4   5   6   7   8   9
      */
    let mut res = HashMap::new();
    //res.insert(1, VecDeque::from(vec!['B', 'W', 'N']));
    res.insert(1, VecDeque::from(vec!['N', 'W', 'B']));
    res.insert(
        2,
        //VecDeque::from(vec!['L', 'Z', 'S', 'P', 'T', 'D', 'M', 'B']),
        VecDeque::from(vec!['B', 'M', 'D', 'T', 'P', 'S', 'Z', 'L']),
    );
    //res.insert(3, VecDeque::from(vec!['Q', 'H', 'Z', 'W', 'R']));
    res.insert(3, VecDeque::from(vec!['R', 'W', 'Z', 'H', 'Q']));
    //res.insert(4, VecDeque::from(vec!['W', 'D', 'V', 'J', 'Z', 'R']));
    res.insert(4, VecDeque::from(vec!['R', 'Z', 'J', 'V', 'D', 'W']));
    //res.insert(5, VecDeque::from(vec!['S', 'H', 'M', 'B']));
    res.insert(5, VecDeque::from(vec!['B', 'M', 'H', 'S']));
    res.insert(
        6,
        //VecDeque::from(vec!['L', 'G', 'N', 'J', 'H', 'D', 'L', 'S']),
        VecDeque::from(vec!['B', 'P', 'V', 'H', 'J', 'N', 'G', 'L']),
    );
    res.insert(
        7,
        //VecDeque::from(vec!['J', 'Q', 'Z', 'F', 'H', 'D', 'L', 'S']),
        VecDeque::from(vec!['S', 'L', 'D', 'H', 'F', 'Z', 'Q', 'J']),
    );
    //res.insert(8, VecDeque::from(vec!['W', 'S', 'F', 'J', 'G', 'Q', 'B']));
    res.insert(8, VecDeque::from(vec!['B', 'Q', 'G', 'J', 'F', 'S', 'W']));
    //res.insert(9, VecDeque::from(vec!['Z', 'W', 'M', 'S', 'C', 'D', 'J']));
    res.insert(9, VecDeque::from(vec!['J', 'D', 'C', 'S', 'M', 'W', 'Z']));
    res
}

fn get_sample_initial_state() -> HashMap<u16, VecDeque<char>> {
    let mut res = HashMap::new();
    res.insert(1, VecDeque::from(vec!['N', 'Z']));
    res.insert(2, VecDeque::from(vec!['D', 'C', 'M']));
    res.insert(3, VecDeque::from(vec!['P']));
    res
}