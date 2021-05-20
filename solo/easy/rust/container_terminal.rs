use std::io;
use std::convert::TryInto;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();

        let chars = line.chars();
        let mut top_containers: Vec<char> = vec![];

        for c in chars {
            if top_containers.len() > 0 {
                let pos = pos_lookup(&top_containers, &c);
                let mut index: i32 = -1;
                match pos {
                    x if x < 0 => {
                        let min = min_over(&top_containers, &c);
                        index = pos_lookup(&top_containers, &min);
                        if index >= 0 {
                            top_containers[index as usize] = c;
                        } else {
                            top_containers.push(c);
                        }
                    },
                    _ => {}
                }
            } else {
                top_containers.push(c)
            }
        }

        println!("{}", top_containers.len())
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");
}

fn pos_lookup(cont: &Vec<char>, c: &char) -> i32 {
    let pos = cont.iter().position(|p| p == c);
    match pos {
        Some(x) => x.try_into().unwrap(),
        _ => -1
    }
}

fn min_over(cont: &Vec<char>, c: &char) -> char {
    let f_cont: Vec<_> = cont.iter().filter(|&v| v > c).collect();
    let ret = f_cont.iter().min();
    match ret {
        Some(&&x) => x,
        _ => 'Z'
    }
}