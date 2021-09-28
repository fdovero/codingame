use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();

    let mut o = 0;
    for i in inputs.split_whitespace() {
        if i == "1" {
            o += 1;
        }
    }
    
    let mut s = 0;
    for i in &inputs.split_whitespace().collect::<Vec<_>>()[0..o] {
        if *i == "0" {
            s += 1;
        }
    }

    println!("{}", s);
}
