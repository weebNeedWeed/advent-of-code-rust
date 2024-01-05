use std::{cmp::Ordering, fs};

fn main() {
    let input = fs::read_to_string("txt1.txt").unwrap();

    let mut back = 0;
    let mut front = 0;
    for c in input.chars() {
        if let Ordering::Equal = c.cmp(&')') {
            back += 1
        }

        if let Ordering::Equal = c.cmp(&'(') {
            front += 1
        }
    }

    println!("result: {}", front - back)
}
