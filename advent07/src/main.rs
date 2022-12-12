use core::panic;
use std::{fs::File, io::{BufReader, BufRead, Lines}};

fn main() {
    let file = File::open("advent07/input").expect("no such file");
    let buf = BufReader::new(file);
    let lines = buf.lines();
    process(lines);
    
}

fn process(mut lines: Lines<BufReader<File>>) {
    let first_line = lines.next().unwrap().unwrap();
    assert_eq!(first_line, "$ cd /");    
    let root_node = finish_node(&mut lines);
    println!("--- RESULT ---");
    println!("below 100000 threshold sum {}", root_node.sum_size_below(100_000));
    println!("smallest above 2_036_703 {}", root_node.smallest_above(2_036_703));
}

fn finish_node(iter: &mut Lines<BufReader<File>>) -> Node {
    let mut child_nodes = vec![];
    let mut total_size = 0;
    loop {
        let s_line = match iter.next() {
            Some(s_line) => s_line.unwrap(),
            None => break,
        };
        if s_line == "$ ls" {
            continue;
        } else if s_line.starts_with("$ cd ") {
            let new_dir = &s_line[5..s_line.len()];
            if new_dir == ".." {
                break;
            } else {
                let dir = finish_node(iter);
                match &dir {
                    Node::Dir(_, size) => {
                        println!("{new_dir} dir : {} ({})", dir.get_size(), size);
                        total_size += size;
                    },
                    Node::File(_) => panic!(),
                }
                child_nodes.push(dir);
            }
        } else if s_line.starts_with("dir ") {
            let name = &s_line[4..s_line.len()-1];
            println!("=> dir {}", name);
        } else if start_with_number(&s_line) {
            let mut split = s_line.split(' ');
            let size = split.next().unwrap().parse::<u64>().unwrap();
            let name = split.next().unwrap();
            let new_node = Node::File(size);
            println!("=> file {}", name);
            child_nodes.push(new_node);
            total_size += size;
        } else {
            panic!()
        }
    }
    Node::Dir(child_nodes, total_size)
}

fn start_with_number(str: &str) -> bool {
    let first_char = str.chars().next().unwrap();
    ('0'..='9').contains(&first_char)
}

#[derive(Debug)]
enum Node {
    Dir(Vec<Node>, u64),
    File(u64),
}

impl Node {
    fn get_size(&self) -> u64 {
        match self {
            Node::Dir(children, _) => {
                let toto: u64 = children.iter().map(|child| child.get_size()).sum();
                toto
            },
            Node::File(size) => *size,
        }
    }

    fn smallest_above(&self, threshold: u64) -> u64 {
        match self {
            Node::Dir(dirs, registered_size) => {
                let mut smallest_above = u64::MAX;
                if *registered_size >= threshold {
                    smallest_above = *registered_size;
                }
                dirs.iter().for_each(|child| {
                    let child_sa = child.smallest_above(threshold);
                    if child_sa < smallest_above {
                        smallest_above = child_sa;
                    }
                });
                smallest_above
            },
            Node::File(_) => {
                u64::MAX
            },
        }
    }

    fn sum_size_below(&self, threshold: u64) -> u64 {
        match self {
            Node::Dir(dirs, registered_size) => {
                let mut sum = 0;
                if *registered_size <= threshold {
                    sum += registered_size;
                } 
                dirs.iter().for_each(|child| sum += child.sum_size_below(threshold));
                sum
            },
            Node::File(_) => {
                0
            },
        }
    }

    // fn add_child(&mut self, child: Node) {
    //     match self {
    //         Node::Dir(children, size) => children.push(child),
    //         _ => panic!()
    //     }
    // }
}


#[cfg(test)]
mod test {
    use super::Node;
    #[test]
    fn test_get_size() {
        let file1 = Node::File(10);
        let file2 = Node::File(20);
        let file3 = Node::File(30);
        let dir1 = Node::Dir(vec![file1], 10);
        let dir2 = Node::Dir(vec![dir1, file2, file3], 60);
        assert_eq!(dir2.get_size(), 60);
    }

    // #[test]
    // fn test_add_child() {
    //     let file1 = Node::File(10);
    //     let dir1 = Node::Dir(vec![file1], 10);
    //     let mut dir2 = Node::Dir(vec![dir1], 10);
    //     let file2 = Node::File(20);
    //     dir2.add_child(file2);
    //     assert_eq!(dir2.get_size(), 30);
    // }
}