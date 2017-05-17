use std::io;

fn main() {
    let temp = get_line()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let (n, hit, t) = (temp[0], temp[1], temp[2]);
    let mut healths = get_line()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    healths.sort();
    println!("{}", get_max_kills(&healths, hit, t));
}

fn get_max_kills(sorted_healths: &Vec<i32>, hit: i32, t: i32) -> i32 {
    let mut shots_left = t;
    let mut num_kills = 0;
    for i in 0..sorted_healths.len() {
        let needed_shots = (sorted_healths[i] as f64 / hit as f64).ceil() as i32;
        if needed_shots > shots_left {
            break;
        } else {
            shots_left -= needed_shots;
            num_kills += 1;
        }
    }
    num_kills
}

fn get_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}
