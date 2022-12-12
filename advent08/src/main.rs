use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn main() {
    let file = File::open("advent08/input").expect("no such file");
    let buf = BufReader::new(file);
    let lines = buf.lines();
    process_1(lines);
    let file = File::open("advent08/input").expect("no such file");
    let buf = BufReader::new(file);
    let lines = buf.lines();
    process_2(lines);
}

fn process_2(lines: Lines<BufReader<File>>) {
    let mut data = vec![];
    let size = 99;
    for line in lines {
        data.push(to_vec(line.unwrap()));
    }
    let mut max_score = 0;
    for i in 0..data.len() {
        for j in 0..size {
            let scenic_score = compute_scenic_score(i,j,&data);
            if scenic_score > max_score {
                max_score = scenic_score;
            }
        }
    }
    println!("Max score: {}", max_score);
}

fn compute_scenic_score(x: usize, y: usize, data: &Vec<Vec<u32>>) -> u32 {
    let height = data.get(x).unwrap().get(y).unwrap();

    let mut trees_above = 0;
    for i in (0..x).rev() {
        let toto = data.get(i).unwrap().get(y as usize).unwrap();
        trees_above += 1;
        if *toto >= *height {
            break;
        }
    }
    let mut trees_below = 0;
    for i in x+1..data.len() {
        let toto = data.get(i).unwrap().get(y as usize).unwrap();
        trees_below += 1;
        if *toto >= *height {
            break;
        }
    }
    let row = data.get(x).unwrap();
    let mut trees_left = 0;
    for i in (0..y).rev() {
        let toto = row.get(i).unwrap();
        trees_left += 1;
        if *toto >= *height {
            break;
        }
    }
    let mut trees_right = 0;
    for i in y+1..row.len() {
        let toto = row.get(i).unwrap();
        trees_right += 1;
        if *toto >= *height {
            break;
        }
    }
    let score=(trees_above*trees_below*trees_left*trees_right) as u32;
    println!("({x},{y}) {height} => trees_above: {trees_above}, trees_below: {trees_below}, trees_left:{trees_left}, trees_right:{trees_right}. Score: {score}");
    score
}

fn process_1(lines: Lines<BufReader<File>>) {
    let mut count = 0;
    let mut data = vec![];
    let size = 99;
    for line in lines {
        data.push(to_vec(line.unwrap()));
    }

    for i in 0..data.len() {
        for j in 0..size {
            if i == 0 || j == 0 || i == data.len()-1 || j == size-1 {
                println!(
                    "tree ({}, {}) is visible because on the edge",
                    i, j
                );
                count += 1;
            } else if is_visible(i, j, &data) {
                count += 1;
            }
        }
    }

    println!("Found {} visible trees", count);
}

fn is_visible(x: usize, y: usize, data: &Vec<Vec<u32>>) -> bool {
    let height = data.get(x).unwrap().get(y).unwrap();

    let mut is_visible_from_above = true;
    for i in 0..x {
        let toto = data.get(i).unwrap().get(y as usize).unwrap();
        // if x==1 && y==4 {
        //     println!(
        //         "tree ({x}, {y}) is compared to ({i},{y}) => {height} VS {toto}",
        //     );
        // }
        if *toto >= *height {
            // println!("tree ({x}, {y}) is hidden by ({i},{y}) => {toto} >= {height}");
            is_visible_from_above = false;
            break;
        }
    }
    if is_visible_from_above {
        println!(
            "tree ({}, {}) with height {} is visible from above",
            x, y, height
        );
        return true;
    }

    let mut is_visible_from_below = true;
    for i in x + 1..data.len() {
        let toto = data.get(i).unwrap().get(y as usize).unwrap();
        if *toto >= *height {
            is_visible_from_below = false;
            break;
        }
    }
    if is_visible_from_below {
        println!(
            "tree ({}, {}) with height {} is visible from below",
            x, y, height
        );
        return true;
    }

    // visible from right or left
    let row = data.get(x).unwrap();

    let mut is_visible_from_left = true;
    for i in 0..y {
        let toto = row.get(i).unwrap();
        if *toto >= *height {
            is_visible_from_left = false;
            break;
        }
    }
    if is_visible_from_left {
        println!(
            "tree ({}, {}) with height {} is visible from left",
            x, y, height
        );
        return true;
    }

    let mut is_visible_from_right = true;
    for i in y + 1..row.len() {
        let toto = row.get(i).unwrap();
        if *toto >= *height {
            is_visible_from_right = false;
            break;
        }
    }
    if is_visible_from_right {
        println!(
            "tree ({}, {}) with height {} is visible from right",
            x, y, height
        );
        return true;
    }

    println!(
        "tree ({}, {}) with height {} is not visible",
        x, y, height
    );

    false
}

fn to_vec(str: String) -> Vec<u32> {
    str.chars().map(|c| c.to_digit(10).unwrap()).collect()
}
