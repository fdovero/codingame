use std::io;
use std::cmp;
use std::convert::TryInto;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let r = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let v = parse_input!(input_line, i32);

    let mut times: Vec<i32> = vec![];
    let mut total = 0;
    for i in 0..v as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let c = parse_input!(inputs[0], i32);
        let n = parse_input!(inputs[1], i32);

        let mut min_index = i;

        if i < r.try_into().unwrap() {
            times.push(0);
        } else {
            min_index = times.iter().position(|&t| t == *times.iter().min().unwrap()).unwrap();
        }

        // combination numbers : 10.pow(n) * 5.pow(c-n) = 5.pow(n) * 2.pow(n) * 5.pow(c) * 5.pow(-n)
        //                                              = 5.pow(c) * 2.pow(n)
        //                                              = 5.pow(c) << n
        times[min_index] += 5_i32.pow(c.try_into().unwrap()) << n;

        total = cmp::max(total, *times.iter().max().unwrap());
    }

    println!("{}", total);
}