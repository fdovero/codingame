use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut count:u32 = 0;
    let mut prev_line = "                ".to_string();

    for i in 0..16 as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();

        let mut inside:bool = false;

        for (i,c) in line.chars().enumerate() {
            let up_char = prev_line.chars().nth(i).unwrap();
            if c == '|' || (c == '+' && (up_char == '|' || up_char == '+')) {
                inside = !inside;
            } else if c == 'o' && inside == true {
                count += 1;
            }
        }

        prev_line = line;
    }

    println!("{}", count);
}
