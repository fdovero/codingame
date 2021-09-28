use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, u64);

    let mut s = n*n;
    for i in 1..=n {
        s-= n % i;
    }

    println!("{}",s);
}