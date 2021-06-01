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
    let l = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, usize);
    let mut wall: Vec<i32> = vec![];
    let mut st: Vec<i32> = vec![];
    let mut ed: Vec<i32> = vec![];
    let mut flag: bool = false;
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        st.push(parse_input!(inputs[0], i32));
        ed.push(parse_input!(inputs[1], i32));

        // paint(&mut wall[st..ed]);
    }

    st.sort();
    ed.sort();

    for x in st.iter() {
        match wall.last() {
            Some(n) => {
                let end = ed.first().unwrap();
                if x > n {
                    wall.push(*x);
                    wall.push(*end);
                } else if end > n {
                    let l = wall.len() - 1;
                    wall[l] = *end;
                }
                ed.remove(0);
            },
            None =>{
                wall.push(*x);
                wall.push(*ed.first().unwrap());
                ed.remove(0);
            }
        }
    }

    for (ind, item) in wall.iter().enumerate().step_by(2) {
        let prev_index = if ind > 0 {ind - 1} else {0};
        if (ind == 0 && item > &0) {
            println!("0 {}", item);
            flag = true;
        }
        if item > &wall[prev_index] {
            println!("{} {}", wall[prev_index], item);
            flag = true;
        }
    }

    if wall.last().unwrap() < &l {
        println!("{} {}", wall.last().unwrap(), l);
        flag = true;
    }

    if !flag {
        println!("All painted");
    }
}

/*fn paint(fence_slice: &mut [char]) {
    for w in fence_slice.iter_mut() {
        *w = 'p'
    }
}*/