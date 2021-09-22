use std::io;
use std::cmp;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    let mut min = u32::MAX;

    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let t = input_line.trim().split(':').collect::<Vec<_>>();

        min = cmp::min(min, t[0].parse::<u32>().unwrap()*3600 + t[1].parse::<u32>().unwrap()*60 + t[2].parse::<u32>().unwrap());
    }

    println!("{:#02}:{:#02}:{:#02}", min/3600, (min/60) % 60, min % 60);
}