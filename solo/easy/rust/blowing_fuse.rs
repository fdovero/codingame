use std::io;

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
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let n = parse_input!(inputs[0], i32);
    let m = parse_input!(inputs[1], i32);
    let c = parse_input!(inputs[2], i32);
    let mut inputs = String::new();
    let mut amperes: Vec<i32> = vec![];
    let mut blown: bool = false;
    io::stdin().read_line(&mut inputs).unwrap();
    for i in inputs.split_whitespace() {
        amperes.push(parse_input!(i, i32));
    }
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    let mut max_ampere = 0;
    let mut cur_ampere = 0;
    let mut switches: Vec<bool> = vec![false; n as usize];
    for i in inputs.split_whitespace() {
        let mx = parse_input!(i, i32);
        match switches[(mx - 1) as usize] {
            true => cur_ampere = cur_ampere - amperes[(mx - 1) as usize],
            false => cur_ampere = cur_ampere + amperes[(mx - 1) as usize]
        }
        switches[(mx - 1) as usize] = !switches[(mx - 1) as usize];
        max_ampere = max_ampere.max(cur_ampere);

        if cur_ampere > c {
            println!("Fuse was blown.");
            blown = true;
            break;
        }
    }

    if !blown {
        println!("Fuse was not blown.");
        println!("Maximal consumed current was {} A.", max_ampere);
    }
}
