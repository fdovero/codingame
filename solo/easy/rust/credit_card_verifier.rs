use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let card = input_line.trim_matches('\n').to_string();

        let n = card.chars().filter(|c| !c.is_whitespace()).enumerate().fold(0, |acc, (i, x)| {
            if i % 2 == 0 {
                let mut ret = x.to_digit(10).unwrap() * 2;
                if ret >=  10 {
                    return acc + ret - 9;
                } else {
                    return acc + ret;
                }
            } else {
                return acc + x.to_digit(10).unwrap();
            }
        });

        let resp = match n % 10 {
            0 => "YES",
            _ => "NO"
        };

        println!("{}", resp);
    }
}