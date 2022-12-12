use std::{collections::{HashMap, VecDeque}, fs::File, io::{BufReader, BufRead, Lines}};

struct State {
    state: HashMap<u16, VecDeque<char>>
}

impl State {
    fn new() -> Self {
        Self {
            state: get_initial_state(),
        }
    }

    // fn get_crate(&mut self, idx: &u16) -> &mut VecDeque<char> {
    //     self.state.get_mut(idx).unwrap()
    // }

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
            println!("moving {} crate(s) from {} to {}", crate_count, origin, dest);

            //let orig_crate = self.state.get_mut(&origin).unwrap();
            //let orig_crate = self.get_crate(&origin);
            //let dest_crate = self.state.get_mut(&dest).unwrap();
            //let dest_crate = self.get_crate(&dest);
            //let toto = self.state.get_many_mut([&origin, &dest]).unwrap();

            // let mut orig_crate_opt = None;
            // let mut dest_crate_opt = None;
            // for (idx, state) in self.state.iter_mut() {
            //     if *idx == origin {
            //         orig_crate_opt = Some(state);
            //     }
            //     if *idx == dest {
            //         dest_crate_opt = Some(state);
            //     }
            // }
            // let orig_crate = orig_crate_opt.unwrap();
            // let dest_crate = dest_crate_opt.unwrap();
            
            // for i in 0..crate_count {
            //     let crate_to_move = orig_crate.pop_front().unwrap();
            //     dest_crate.push_front(crate_to_move);
            // }

            self.print_tops();
        }
    }

    fn print_tops(&self) {
        for (idx, state) in self.state.iter() {
            print!("{}: \"{}\" | ", idx, state.front().unwrap())
        }
        println!()
    }
}

fn main() {
    let file = File::open("advent05/input").expect("no such file");
    let buf = BufReader::new(file);
    let lines = buf.lines();
    let mut state = State::new();
    state.execute_moves(lines);
    
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
  res.insert(1, VecDeque::from(vec!['B', 'W', 'N']));
  res.insert(2, VecDeque::from(vec!['L', 'Z', 'S', 'P', 'T', 'D', 'M', 'B']));
  res.insert(3, VecDeque::from(vec!['Q', 'H', 'Z', 'W', 'R']));
  res.insert(4, VecDeque::from(vec!['W', 'D', 'V', 'J', 'Z', 'R']));
  res.insert(5, VecDeque::from(vec!['S', 'H', 'M', 'B']));
  res.insert(6, VecDeque::from(vec!['L', 'G', 'N', 'J', 'H', 'D', 'L', 'S']));
  res.insert(7, VecDeque::from(vec!['J', 'Q', 'Z', 'H', 'D', 'L', 'S']));
  res.insert(8, VecDeque::from(vec!['W', 'S', 'F', 'J', 'G', 'Q', 'B']));
  res.insert(3, VecDeque::from(vec!['Z', 'W', 'M', 'S', 'C', 'D', 'J']));
  res
}