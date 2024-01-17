use std::mem::size_of;
use std::{fs, thread};

#[derive(Debug)]
struct Grid {
    cells: [[u32; 1000]; 1000],
}

impl Grid {
    fn new() -> Grid {
        Grid {
            cells: [[0; 1000]; 1000],
        }
    }

    fn toggle_cell(&mut self, row: usize, col: usize) {
        self.cells[row][col] = 1 - self.cells[row][col]
    }

    fn turn_on_cell(&mut self, row: usize, col: usize) {
        self.cells[row][col] = 1
    }

    fn turn_off_cell(&mut self, row: usize, col: usize) {
        self.cells[row][col] = 0
    }

    fn count_turning_on_light(&self) -> u32 {
        self.cells.iter().flatten().fold(0, |acc, x| acc + x)
    }
}

#[derive(Debug)]
enum Command {
    Toggle(usize, usize, usize, usize),
    TurnOn(usize, usize, usize, usize),
    TurnOff(usize, usize, usize, usize),
}

impl Command {
    fn get_from_and_to(from: &str, to: &str) -> (usize, usize, usize, usize) {
        let from = from
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let to = to
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        (from[0], from[1], to[0], to[1])
    }

    fn build(input: &str) -> Command {
        let args = input.split(' ').collect::<Vec<&str>>();
        let command = match args[0] {
            "toggle" => {
                let (fr, fc, tr, tc) = Command::get_from_and_to(args[1], args[3]);
                Command::Toggle(fr, fc, tr, tc)
            }

            "turn" => {
                let (fr, fc, tr, tc) = Command::get_from_and_to(args[2], args[4]);
                match args[1] {
                    "on" => Command::TurnOn(fr, fc, tr, tc),
                    "off" => Command::TurnOff(fr, fc, tr, tc),
                    _ => Command::Toggle(1, 1, 1, 1),
                }
            }

            _ => Command::Toggle(1, 1, 1, 1),
        };
        command
    }
}

fn main() {
    thread::Builder::new()
        .stack_size(size_of::<u32>() * 1000 * 1000 * 20)
        .spawn(|| {
            let input = fs::read_to_string("test.txt").unwrap();
            let mut grid = Grid::new();
            for line in input.lines() {
                let command = Command::build(line.trim());
                if let Command::TurnOn(fr, fc, tr, tc) = command {
                    for i in fr..=tr {
                        for j in fc..=tc {
                            grid.turn_on_cell(i, j)
                        }
                    }
                }
                if let Command::Toggle(fr, fc, tr, tc) = command {
                    for i in fr..=tr {
                        for j in fc..=tc {
                            grid.toggle_cell(i, j)
                        }
                    }
                }
                if let Command::TurnOff(fr, fc, tr, tc) = command {
                    for i in fr..=tr {
                        for j in fc..=tc {
                            grid.turn_off_cell(i, j)
                        }
                    }
                }
            }

            println!("{:?}", grid.count_turning_on_light());
        })
        .unwrap()
        .join()
        .unwrap();
}
