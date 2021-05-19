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
    let mut r_1 = parse_input!(input_line, u32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut r_2 = parse_input!(input_line, u32);

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    loop {
        match (r_1, r_2) {
            (r1, r2) if r1 < r2 => r_1 = river_next(r_1),
            (r1, r2) if r1 == r2 => { println!("{}", r_1); break },
            (r1, r2) if r1 > r2 => r_2 = river_next(r_2),
            _ => { println!("Massive error"); break }
        }
        // eprintln!("{} {}", r_1, r_2);
    }
}

fn river_next(river: u32) -> u32 {
    let digits: Vec<_> = river.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let mut sum = 0;
    for digit in digits {
        sum = sum + digit
    }
    river + sum
}