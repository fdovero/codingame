use std::io;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {

    // game loop
    loop {
        let mut mountains = Vec::new();
     
        for i in 0..8 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            mountains.push(parse_input!(input_line, i32)); // represents the height of one mountain, from 9 to 0.
        }

        let mut max = mountains.iter().max().unwrap();
        let mut ind = mountains.iter().position(|m| m == max).unwrap();
        
        // mountains.binary_search(&mountains.iter().max().unwrap()).unwrap()
        // Write an action using println!("message...");
        // To debug: print_err!("Debug message...");
        //print_err!("{:?}, {}", mountains, max);
        println!("{}",ind); // The number of the mountain to fire on.
    }
}
