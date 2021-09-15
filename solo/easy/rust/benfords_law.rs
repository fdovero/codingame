use std::io;
use regex::Regex;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    let mut v = vec![0;9];
    let benford = vec![30.1,17.6,12.5,9.7,7.9,6.7,5.8,5.1,4.6];

    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let transaction = input_line.trim_matches('\n').to_string();

        let re = Regex::new(r"[1-9]").unwrap();
        let e = re.captures(&transaction).unwrap().get(0).unwrap().as_str().parse::<usize>().unwrap() - 1;

        v[e] = v[e] + 1;
    }

    let t = v.iter().enumerate().all(|(i, x)| &benford[i] - 10.0 <= (*x as f32 * 100.0 / n as f32) && (*x as f32 * 100.0 / n as f32) <= &benford[i] + 10.0);

    println!("{:?}", !t);
}