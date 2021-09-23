use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let (mut a, mut b) = (parse_input!(inputs[0], i32), parse_input!(inputs[1], i32));
    
    if a < b {
        let c = b;
        b = a;
        a = c;
    }

    println!("{} * {}", a, b);

    let mut sum = 0;
    let mut rest = "".to_string();

    while b > 0 {
        if b % 2 == 1 {
            rest = format!("{} + {}", rest, a);
            sum += a;
            b -= 1;
        } else {
            a *= 2;
            b /= 2;
        }

        println!("= {} * {}{}", a, b, rest);
    }

    println!("= {}", sum);
}
