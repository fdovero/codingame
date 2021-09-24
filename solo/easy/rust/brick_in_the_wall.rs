use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let x = parse_input!(input_line, usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, usize);
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();

    let mut bricks: Vec<u32> = inputs.split_whitespace().map(|b| b.parse::<u32>().unwrap()).collect();
    bricks.sort();
    bricks.reverse();
    let mut row = 0.0;
    let mut time = 0.0;
    while bricks.len() > 0 {
        let nb = x.min(bricks.len());
        time += bricks.drain(..nb).fold(0_f64,|acc, w| acc + w as f64 * row * 0.65);
        row += 1.0;
    }

    println!("{:.3}", time);
}