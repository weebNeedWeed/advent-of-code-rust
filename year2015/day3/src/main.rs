use std::fs;

const WIDTH: i32 = 500;

#[derive(Debug)]
struct Santa {
    x: i32,
    y: i32,
    visited: Vec<i32>,
}

impl Santa {
    fn new() -> Santa {
        Santa {
            x: 0,
            y: 0,
            visited: vec![0],
        }
    }

    fn move_up(&mut self) {
        self.y -= 1
    }

    fn move_left(&mut self) {
        self.x -= 1
    }

    fn move_right(&mut self) {
        self.x += 1
    }

    fn move_down(&mut self) {
        self.y += 1
    }

    fn is_visited_current_point(&self) -> bool {
        self.visited.iter().find(|&&x| {
            let idx = self.get_1d_point(self.x, self.y);
            x == idx
        }) != None
    }

    fn get_1d_point(&self, x: i32, y: i32) -> i32 {
        (x * WIDTH) + y
    }

    fn visit_current_point(&mut self) {
        self.visited.push(self.get_1d_point(self.x, self.y))
    }
}

fn main() {
    let input = fs::read_to_string("y152.txt").unwrap();
    let mut santa = Santa::new();
    let mut house = 1;
    for dir in input.chars() {
        match dir {
            '^' => santa.move_up(),
            '>' => santa.move_right(),
            'v' => santa.move_down(),
            '<' => santa.move_left(),
            _ => continue,
        }

        if !santa.is_visited_current_point() {
            santa.visit_current_point();
            house += 1
        }
    }

    println!("House = {house}")
}
