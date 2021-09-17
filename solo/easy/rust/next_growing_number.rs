use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = input_line.trim_matches('\n').to_string();

    let number = (n.parse::<u64>().unwrap() + 1).to_string();
    let mut v = vec![];

    let mut fl = false;

    eprintln!("{}", n);
    eprintln!("{}", number);

    number.chars().enumerate().for_each(|(i, c)| {
        match i {
            0 => v.push(c),
            _ => {
                if c < v[i-1] || fl == true { 
                    v.push(v[i-1]);
                    fl = true; 
                } else { 
                    v.push(c); 
                }
            }
        };
    });

    println!("{}", v.into_iter().collect::<String>());
}