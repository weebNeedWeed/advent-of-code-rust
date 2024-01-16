use std::fs;

fn main() {
    let input = fs::read_to_string("test.txt").unwrap();
    let mut count = 0;
    for line in input.lines() {
        if is_nice(&line.trim()) {
            count += 1
        }
    }

    println!("count = {count}");
}

fn is_nice(input: &str) -> bool {
    contains_three_vowels(input)
        && contains_one_letter_appearing_twice(input)
        && not_contains_forbidden_string(input)
}

fn contains_three_vowels(input: &str) -> bool {
    input
        .matches(&['a', 'o', 'e', 'u', 'i'])
        .collect::<Vec<_>>()
        .len()
        >= 3
}

fn contains_one_letter_appearing_twice(input: &str) -> bool {
    let mut prev = ' ';
    for c in input.chars() {
        if c == prev {
            return true;
        }
        prev = c
    }

    false
}

fn not_contains_forbidden_string(input: &str) -> bool {
    let allowed = ["ab", "cd", "pq", "xy"];
    let mut prev = ' ';
    for word in input.chars() {
        let pair = format!("{}{}", prev, word);
        if allowed.contains(&&pair[..]) {
            return false;
        }
        prev = word
    }
    true
}
