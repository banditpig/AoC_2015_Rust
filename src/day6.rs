use utils::read_file;
use std::thread;

trait Switchable<T> {
    fn switch(&mut self, op: &Op) -> GenLights<T>;
    fn result(&mut self) -> i64;
}

type GenLights<T> = [[T; 1000]; 1000];
type Lights = GenLights<State>;
type DimmerLights = GenLights<i8>;


impl Switchable<i8> for DimmerLights {
    fn switch(&mut self, op: &Op) -> DimmerLights {
        match op {
            &Op::SwitchOn { ref start, ref end } => {
                for r in start.row..end.row + 1 {
                    for c in start.col..end.col + 1 {
                        let v = self[r][c];
                        self[r][c] = v + 1;
                    }
                }
            }
            &Op::SwitchOff { ref start, ref end } => {
                for r in start.row..end.row + 1 {
                    for c in start.col..end.col + 1 {
                        let mut v = self[r][c] - 1;
                        if v < 0 { v = 0 }
                        self[r][c] = v;
                    }
                }
            }
            &Op::SwitchToggle { ref start, ref end } => {
                for r in start.row..end.row + 1 {
                    for c in start.col..end.col + 1 {
                        let v = self[r][c];

                        self[r][c] = v + 2;
                    }
                }
            }
        }
        *self
    }
    fn result(&mut self) -> i64 {
        let mut sum: i64 = 0;
        self.iter().for_each(|s| s.iter().for_each(|v| sum = sum + (*v as i64)));
        sum
    }
}

impl Switchable<State> for Lights {
    fn switch(&mut self, op: &Op) -> Lights {
        match op {
            &Op::SwitchOn { ref start, ref end } => {
                for r in start.row..end.row + 1 {
                    for c in start.col..end.col + 1 {
                        self[r][c] = State::On;
                    }
                }
            }
            &Op::SwitchOff { ref start, ref end } => {
                for r in start.row..end.row + 1 {
                    for c in start.col..end.col + 1 {
                        self[r][c] = State::Off;
                    }
                }
            }
            &Op::SwitchToggle { ref start, ref end } => {
                for r in start.row..end.row + 1 {
                    for c in start.col..end.col + 1 {
                        let mut old = self[r][c];
                        match old {
                            State::Off => self[r][c] = State::On,
                            State::On => self[r][c] = State::Off
                        }
                    }
                }
            }
        }
        *self
    }
    fn result(&mut self) -> i64 {
        let mut on_cnt: i64 = 0;
        self.iter().for_each(|s| s.iter().for_each(|st| if *st == State::On { on_cnt += 1; }));

        on_cnt
    }
}

#[derive(Debug)]
struct Location {
    row: usize,
    col: usize
}

#[derive(Debug)]
#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    On,
    Off
}

#[derive(Debug)]
enum Op {
    SwitchOn { start: Location, end: Location },
    SwitchOff { start: Location, end: Location },
    SwitchToggle { start: Location, end: Location }
}
type AllOps = Vec<Op>;

fn parse_line(line: &str) -> Op {
    if line.starts_with("turn on ") {
        let vec: Vec<&str> = line[8..].split(" through ").collect::<Vec<&str>>();
        let op = Op::SwitchOn { start: loc_from_string(vec[0]), end: loc_from_string(vec[1]) };
        return op;
    }
    if line.starts_with("turn off ") {
        let vec: Vec<&str> = line[9..].split(" through ").collect::<Vec<&str>>();
        let op = Op::SwitchOff { start: loc_from_string(vec[0]), end: loc_from_string(vec[1]) };
        return op;
    }

    let vec: Vec<&str> = line[7..].split(" through ").collect::<Vec<&str>>();

    let op = Op::SwitchToggle { start: loc_from_string(vec[0]), end: loc_from_string(vec[1]) };
    return op;
}

fn run_ops_gen<'a, T>(ops: &AllOps, lights: &'a mut Switchable<T>) -> &'a mut Switchable<T> {
    for op in ops {
        lights.switch(&op);
    }
    lights
}


fn loc_from_string(s: &str) -> Location {
    let vec = s.split(",").collect::<Vec<&str>>();
    let r = vec[0].parse::<usize>().unwrap();
    let c = vec[1].parse::<usize>().unwrap();
    let loc = Location { row: r, col: c };
    loc
}

fn init() -> Lights {
    [[State::Off; 1000]; 1000]
}

fn init2() -> DimmerLights {
    [[0; 1000]; 1000]
}

pub fn part1_part2() {
    let s = read_file("data/day6.txt".to_string());
    let mut all_ops: Vec<Op> = vec![];
    s.lines().for_each(|line| all_ops.push(parse_line(&line)));
    part1(&all_ops);
    part2(&all_ops);
}

fn part2(all_ops: &AllOps) {
    let mut lights: [[i8; 1000]; 1000] = init2();
    let sum = run_ops_gen(&all_ops, &mut lights).result();
    println!("Day 6 part 2 {}", sum);
}

fn part1(all_ops: &AllOps) {
    let mut lights: [[State; 1000]; 1000] = init();
    let sum: i64 = run_ops_gen(&all_ops, &mut lights).result();
    println!("Day 6 part 1 {}", sum);
}



