use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let input = input_line.trim_matches('\n').to_string();

    let mut nums: Vec<char>= input.chars().filter(|n| n != &'.' && n != &'-' && n != &' ').collect();
    nums.sort();

    match input.contains("-") {
        true => {
            if input.contains(".") {
                nums.insert(1, '.');
            }
            nums.insert(0, '-');
        },
        false => {
            nums.reverse();
            if input.contains(".") {
                nums.insert(nums.len() - 1, '.');
            }
        }
    }

    println!("{}", nums.into_iter().collect::<String>().parse::<f64>().unwrap());
}
