use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    
    let mut f = 0;
    let mut b = 0;

    let mut p = 0;
    let mut fcounter = 0;
    let mut bcounter = 0;
    let mut fbcounter = 0;

    let mut fb_flag = false;


    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();

        match &line[..] {
            "Fizz" => {
                p = p + 1;
                if f == 0 {
                    f = p;
                } else {
                    f = fcounter;
                }
                fcounter = 0;
            },
            
            "Buzz" => {
                p = p + 1;
                if b == 0 {
                    b = p;
                } else {
                    b = bcounter;
                }
                bcounter = 0;
            },

            "FizzBuzz" => {
                p = p + 1;
                if fb_flag == false {
                    fb_flag = true;
                } else {
                    if f + b == 0 {
                        f = fbcounter;
                        b = fbcounter;
                    } else if f == 0 {
                        f = fbcounter;
                        bcounter = 0;
                    } else if b == 0 {
                        b = fbcounter;
                        fcounter = 0;
                    }
                }
                fbcounter = 0;
            },
            
            _ => {
                let np = line.parse::<u32>().unwrap();
                if np != p + 1 && p != 0 {
                    if b == p {
                        b = np - 1;
                    }
                    if f == p {
                        f = np -1;
                    }
                }
                p = np;
            }
        };

        fcounter = fcounter + 1;
        bcounter = bcounter + 1;
        fbcounter = fbcounter + 1;

        eprintln!("{} {} {}", p, f, b);
    }

    println!("{} {}", f, b);
}
