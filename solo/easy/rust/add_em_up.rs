use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, usize);
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();

    let mut cards: Vec<u32> = inputs.split_whitespace().map(|c| c.parse::<u32>().unwrap()).collect();
    cards.sort();

    let mut cost = 0;
    while cards.len() > 1 {
        let new_card = cards[0] + cards[1];
        cost += new_card;
        cards.remove(0);
        cards.remove(0);
        cards.push(new_card);
        cards.sort();
    }

    println!("{}", cost );
}
