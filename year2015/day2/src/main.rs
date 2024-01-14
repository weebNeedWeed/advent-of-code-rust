use std::cmp;
use std::fs;

fn main() {
    let input = fs::read_to_string("y152.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let mut v = line.split('x');
        let l: u32 = v.next().unwrap().trim().parse().unwrap();
        let w: u32 = v.next().unwrap().trim().parse().unwrap();
        let h: u32 = v.next().unwrap().trim().parse().unwrap();

        let surface_area = 2 * l * w + 2 * w * h + 2 * l * h;
        let min_area = cmp::min(cmp::min(l * w, w * h), l * h);
        sum += surface_area + min_area;
    }

    println!("{sum}")
}
