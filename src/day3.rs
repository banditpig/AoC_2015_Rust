use file_utils::read_file;
use std::collections::HashMap;


pub fn part1_part2() {
    let s = read_file("data/day3.txt".to_string());

    part1(&s);
    part2(&s);
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
}

fn part2(str: &String) {
    let  mut visited = HashMap::new();
    let start = Pos::new(0, 0);
    visited.insert(start, 2);

    let mut x = 0;
    let mut y = 0;

    let mut ix = 0;
    let bytes = str.as_bytes();

    while ix < bytes.len() {
        match bytes[ix]{
            94  => y = y - 1,
            118 => y = y + 1,
            60  => x = x - 1,
            62  => x = x + 1,
            _   => x = x + 0,
        }
        let newpos = Pos::new(x, y);

        match visited.get(&newpos).cloned() {
            None    => { visited.insert(newpos, 1);     }
            Some(v) => { visited.insert(newpos, v + 1); }
        }
        ix += 2;

    }
    x = 0;
    y = 0;
    ix = 1;
    while ix < bytes.len() {
        match bytes[ix]{
            94  => y = y - 1,
            118 => y = y + 1,
            60  => x = x - 1,
            62  => x = x + 1,
            _   => x = x + 0,
        }
        let newpos = Pos::new(x, y);

        match visited.get(&newpos).cloned() {
            None    => { visited.insert(newpos, 1);     }
            Some(v) => { visited.insert(newpos, v + 1); }
        }
        ix += 2;

    }



    println!("xx {:?}", visited.len());
}
fn part1(str: &String) {
    let  mut visited = HashMap::new();
    let start = Pos::new(0, 0);
    visited.insert(start, 1);

    let mut x = 0;
    let mut y = 0;

    for c in str.chars() {
        match c {
            '^' => y = y - 1,
            'v' => y = y + 1,
            '<' => x = x - 1,
            '>' => x = x + 1,
            _   => x = x + 0,
        }
        let newpos = Pos::new(x, y);

        match visited.get(&newpos).cloned() {
            None    => { visited.insert(newpos, 1);     }
            Some(v) => { visited.insert(newpos, v + 1); }
        }
    }

    println!("xx {:?}", visited.len());
}

