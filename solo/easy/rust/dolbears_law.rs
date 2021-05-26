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
    let m = parse_input!(input_line, i32);
    let mut n60: Vec<f32> = vec![];
    let mut n8: Vec<f32> = vec![];
    let mut full: Vec<f32> = vec![];
    for i in 0..m as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();

        let mut l: Vec<f32> = line.split(' ').map(|x| x.parse::<f32>().unwrap()).collect();
        n60.push(10.0 + (l.iter().sum::<f32>() - 40.0) / 7.0);

        full.extend(l);
    }
    println!("{:.1}", average(&n60));

    if average(&n60) <= 30.0 && average(&n60) >= 5.0 {
        if full.len() % 2 == 1 {
            full.pop();
        }
        n8.push(average(&full.chunks(2).map(|x| x.iter().sum::<f32>() + 5.0).collect::<Vec<f32>>()));
        println!("{:.1}", average(&n8));
    }
}

fn average(n: &Vec<f32>) -> f32 {
    n.iter().sum::<f32>() / n.len() as f32
}