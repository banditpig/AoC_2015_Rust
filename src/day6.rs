use utils::read_file;
trait Switchable<T> {
    fn switch(& mut self, op:&Op) -> GenLights<T>;
}
type GenLights<T> = [[T; 1000]; 1000];
type Lights = [[State; 1000]; 1000];
type DimmerLights = [[u32; 1000]; 1000];


//impl Switchable<u32> for DimmerLights{
//    fn switch(& mut self, op: &Op, ) ->  GenLights<u32>{
//        *self
//    }
//}

impl Switchable<State> for Lights{
    fn switch(& mut self, op: &Op) -> Lights{
        match op {
            &Op::switchOn { ref start, ref end } => {
                for r in start.row..end.row + 1 {
                    for c in start.col..end.col + 1 {
                        self[r][c] = State::On;
                    }
                }
            }
            &Op::switchOff { ref start, ref end } => {
                for r in start.row..end.row + 1 {
                    for c in start.col..end.col + 1 {
                        self[r][c] = State::Off;
                    }
                }
            }
            &Op::switchToggle { ref start, ref end } => {
                for r in start.row..end.row + 1 {
                    for c in start.col..end.col + 1 {
                        let mut old = self[r][c];
                        match old {
                            State::Off => self[r][c] = State::On,
                            State::On =>  self[r][c] = State::Off
                        }

                    }
                }
            }
        }
        *self
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


fn run_ops<'a>(ops: & AllOps, lights: &'a mut Lights) -> &'a mut Lights {

    for op in ops{
        lights.switch(&op);
    }
   lights
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



