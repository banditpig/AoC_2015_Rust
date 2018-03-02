use file_utils::read_file;

type Lights = [[State; 1000]; 1000];

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
    switchOn { start: Location, end: Location },
    switchOff { start: Location, end: Location },
    switchToggle { start: Location, end: Location }
}
type AllOps = Vec<Op>;

fn parse_line(line: &str) -> Op{


    if line.starts_with("turn on "){
        let vec: Vec<&str> =  line[8..].split(" through ").collect::<Vec<&str>>();;
        let op = Op::switchOn {start: loc_from_string(vec[0]), end: loc_from_string(vec[1])};
        return op;
    }
    if line.starts_with("turn off "){
        let vec: Vec<&str> =  line[9..].split(" through ").collect::<Vec<&str>>();;
        let op = Op::switchOff {start: loc_from_string(vec[0]), end: loc_from_string(vec[1])};
        return op;
    }

    let vec: Vec<&str> =  line[7..].split(" through ").collect::<Vec<&str>>();;
    let op = Op::switchToggle {start: loc_from_string(vec[0]), end: loc_from_string(vec[1])};
    return op;

}

fn run_op<'a>(op: &'a Op, lights: &'a mut Lights) -> &'a mut Lights {
    match op {
        &Op::switchOn { ref start, ref end } => {
            for r in start.row..end.row + 1 {
                for c in start.col..end.col + 1 {
                    lights[r][c] = State::On;
                }
            }
        }
        &Op::switchOff { ref start, ref end } => {
            for r in start.row..end.row + 1 {
                for c in start.col..end.col + 1 {
                    lights[r][c] = State::Off;
                }
            }
        }
        &Op::switchToggle { ref start, ref end } => {
            for r in start.row..end.row + 1 {
                for c in start.col..end.col + 1 {
                    let mut old = lights[r][c];
                    match old {
                        State::Off => lights[r][c] = State::On,
                        State::On =>  lights[r][c] = State::Off
                    }

                }
            }
        }
    }
    lights
}

fn run_ops<'a>(ops: &'a AllOps, lights: &'a mut Lights) -> &'a Lights {
    let  ret = ops.iter()
        .fold(lights, |lts, op| run_op(op, lts));
    ret
}

fn loc_from_string(s:&str) -> Location{
    let vec = s.split(",").collect::<Vec<&str>>();
    let r = vec[0].parse::<usize>().unwrap();
    let c = vec[1].parse::<usize>().unwrap();
    let loc = Location{row: r, col:c};
    loc
}
fn init() -> Lights {
    [[State::Off; 1000]; 1000]
}

fn count_states(lights:  &Lights) -> (usize, usize){
    let mut on_cnt = 0 ;
    for r in 0..1000 {
        for c in 0..1000{
            if lights[r][c] == State::On {
                on_cnt += 1;
            }
        }
    }
    (on_cnt, 1000000 - on_cnt)
}

pub fn part1_part2(){
    let s = read_file("data/day6.txt".to_string());
    part1(&s);
    part2(&s);
}

fn part2(s: &String) {

}
fn part1(s: &String) {
    let mut all_ops: Vec<Op> = vec![];
    let mut lights:[[State;1000];1000] = init();

    for line in s.lines(){
        all_ops.push(parse_line(&line)) ;
    }

    println!("{:?}", count_states(run_ops(&all_ops,&mut lights)));
}



