use std::cmp::Ordering;
use std::io;

pub fn exetute_compare() {
    let mut alice = String::new();
    let mut bob = String::new();
    io::stdin().read_line(&mut alice).ok();
    io::stdin().read_line(&mut bob).ok();
    let (a_p, b_p) = compute(&alice, &bob);
    println!("{} {}", a_p, b_p);
}

fn compute(alice: &str, bob: &str) -> (u32, u32) {
    let mut a_p = 0;
    let mut b_p = 0;
    alice
        .split_whitespace()
        .zip(bob.split_whitespace())
        .map(|(a, b): (&str, &str)| (a.parse().unwrap(), b.parse().expect("se")))
        .for_each(|(a, b): (u32, u32)| match a.cmp(&b) {
            Ordering::Greater => a_p += 1,
            Ordering::Equal => (),
            Ordering::Less => b_p += 1,
        });
    (a_p, b_p)
}

#[test]
fn counts() {
    assert_eq!((1, 2), compute(&"2 3 4", &"5 4 1"));
    assert_eq!((0, 3), compute(&"0 0 0", &"5 4 1"));
    assert_eq!((0, 0), compute(&"1 1 1", &"1 1 1"));
    assert_eq!((3, 0), compute(&"4 5 20", &"3 4 1"));
}
