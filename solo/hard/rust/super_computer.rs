use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Calc {
    s: u32,
    e: u32
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, u32);
    let mut calcs: Vec<Calc> = vec![];
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let j = parse_input!(inputs[0], u32);
        let d = parse_input!(inputs[1], u32);
        calcs.push(Calc {s: j, e: j + d - 1});
    }

    calcs.sort_by(|a, b| a.e.cmp(&b.e));

    let mut count = 1;
    let mut min = calcs[0].e;

    for c in &calcs[1..] {
        if min < c.s {
            min = c.e;
            count += 1;
        }
    }

    println!("{}", count);
}