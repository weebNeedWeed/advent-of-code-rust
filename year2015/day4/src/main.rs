use std::fs;

fn main() {
    let input = fs::read_to_string("test.txt").unwrap();
    let mut count = 1;
    loop {
        let input = format!("{}{}", input.trim(), count);
        let output = format!("{:x}", md5::compute(&input.as_bytes()));
        let zero_vec = output
            .bytes()
            .take_while(|&b| b == b'0')
            .collect::<Vec<u8>>();
        if zero_vec.len() == 5 {
            println!("{count}, {input}");
            break;
        }
        count += 1
    }
}
