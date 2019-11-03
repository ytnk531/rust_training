use std::cmp::Ordering;
use std::io;
pub fn exetute_compare() {
    let mut alice = String::new();
    let mut bob = String::new();
    io::stdin().read_line(&mut alice).ok();
    io::stdin().read_line(&mut bob).ok();
    let mut a_p = 0;
    let mut b_p = 0;
    alice
        .split_whitespace()
        .zip(bob.split_whitespace())
        .map(|(a, b): (&str, &str)| (a.parse().expect("su"), b.parse().expect("se")))
        .for_each(|(a, b): (u32, u32)| match a.cmp(&b) {
            Ordering::Greater => a_p += 1,
            Ordering::Equal => (),
            Ordering::Less => b_p += 1,
        });
    println!("{} {}", a_p, b_p);
}
