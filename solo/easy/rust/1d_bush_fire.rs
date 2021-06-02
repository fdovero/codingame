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
        let mut line: Vec<char> = input_line.trim_matches('\n').chars().collect();
        let mut drop_count: i32 = 0;
        loop {
            let fire_pos = line.iter().position(|x| x == &'f');
            match fire_pos {
                Some(x) => {
                    drop_water(&mut line, x + 1);
                    drop_count = drop_count + 1;
                },
                None => {
                    println!("{}", drop_count);
                    break;
                }
            }
        }
    }
}

fn drop_water(bushslice: &mut Vec<char>, drop_index: usize) {
    let low_index = (drop_index-1).max(0);
    let high_index = (drop_index+1).min(bushslice.len()-1);
    for el in bushslice[low_index..=high_index].iter_mut() {
        *el = '.';
    }
}