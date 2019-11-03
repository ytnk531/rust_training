// https://www.hackerrank.com/challenges/plus-minus/problem
use std::cmp::Ordering;
use std::io;

pub fn execute() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let ll: f32 = buf.trim().parse().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let mut z = 0f32;
    let mut g = 0f32;
    let mut l = 0f32;
    let arr = buf.split_whitespace().map(|s| s.parse().unwrap());
    arr.for_each(|n: i32| match n.cmp(&0) {
        Ordering::Equal => z += 1f32,
        Ordering::Greater => g += 1f32,
        Ordering::Less => l += 1f32,
    });
    println!("{:.*}", 6, g / ll);
    println!("{:.*}", 6, l / ll);
    println!("{:.*}", 6, z / ll);
}
