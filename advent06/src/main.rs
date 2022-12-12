use std::{collections::{VecDeque, HashSet}, fs::File, io::{BufReader, Read}, str::Chars};

fn main() {
    let file = File::open("advent06/input").expect("no such file");
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    buf.read_to_string(&mut line).expect("cannot read string");
    println!("4: {}", find_pos(4, line.chars()));
    println!("14: {}", find_pos(14, line.chars()));
}

fn find_pos(capacity: usize, chars: Chars) -> usize {
    let mut last_four: VecDeque<char> = VecDeque::with_capacity(capacity);
    for (pos, cha) in chars.enumerate() {
        if last_four.len() < capacity {
            last_four.push_front(cha)
        } else {
            last_four.pop_back();
            last_four.push_front(cha);
        }
        if last_four.len() == capacity && has_unique_elements(&last_four) {
            println!("last four: {:?}", last_four);
            println!("cha {}", cha);
            return pos + 1;
        }
        
    }
    panic!()
}

fn has_unique_elements(iter: &VecDeque<char>) -> bool
{
    let mut uniq = HashSet::new();
    iter.iter().all(move |x| uniq.insert(x))
}

#[cfg(test)]
mod test {
    use crate::find_pos;

    #[test]
    fn test_find_pos() {
        assert_eq!(find_pos(4, "bvwbjplbgvbhsrlpgdmjqwftvncz".chars()), 5);
        assert_eq!(find_pos(14, "bvwbjplbgvbhsrlpgdmjqwftvncz".chars()), 23);
    }
}