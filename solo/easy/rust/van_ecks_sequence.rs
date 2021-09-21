use std::io;
use std::collections::HashMap;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let a1 = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    let mut seen = HashMap::new();
    let mut next = a1;

    for i in 1..n {
        let s = seen.entry(next).or_insert(i);

        if *s == i {
            next = 0;
        } else {
            next = i - *s;
            *s = i;
        }
    }

    println!("{}", next);
}