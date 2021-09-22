use std::io;
use std::cmp;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Horse {
    v: i32,
    e: i32
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut v_horse: Vec<Horse> = vec![];
    let mut min = i32::MAX;
   
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let v = parse_input!(inputs[0], i32);
        let e = parse_input!(inputs[1], i32);
        let horse = Horse {v, e};

        min = v_horse.iter().fold(min, |acc, hi| cmp::min(acc,ecart(hi, &horse)));

        v_horse.push(horse);
    }


    println!("{}", min);
}

fn ecart (h1: &Horse, h2: &Horse) -> i32 {
    (h2.v - h1.v).abs() + (h2.e - h1.e).abs()
}