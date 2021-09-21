use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, u32);

    let mut i = 2;
    let mut acc = n + 1;
    let mut s = 0;
    let mut ret = 0;

    while s < n*n {
        let dec = match i % 4 {
            0 => 1,
            _ => 0
        };
        ret = ret + i/2;
        s = i * n - ret + dec;
        acc = acc + s;

        i = i + 1;
    }

    println!("{}", acc);
}