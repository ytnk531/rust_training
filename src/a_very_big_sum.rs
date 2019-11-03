use std::io;

pub fn execute() {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).ok();
    buf.clear();
    io::stdin().read_line(&mut buf).ok();
    let result: u64 = buf.split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .fold(0, |sum, x| sum + x);
    println!("{}", result);
}
