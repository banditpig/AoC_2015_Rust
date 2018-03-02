use std::cmp;
use file_utils::read_file;

pub fn part1_part2() {
    let s = read_file("data/day2.txt".to_string());
    part1(&s);
    part2(&s);
}
fn part2(s: &String){
    let mut sum_area = 0;
    for line in s.lines() {
        let tpl = split(line.to_string());
        sum_area = sum_area + area2(sort(tpl));

    }
    println!("Day 2 part 2 {}", sum_area);
}

fn part1(s: &String){
    let mut sum_area = 0;
    for line in s.lines() {
        let tpl = split(line.to_string());
        sum_area = sum_area + area(tpl);

    }
    println!("Day 2 part 1 {}", sum_area);
}


pub fn sort<T: Eq + Ord + Clone>(tpl: (T, T, T) ) -> (T, T, T){
    let (a, b, c) = tpl;
    let mut v = [a, b, c];
    v.sort();

    (v[0].clone(), v[1].clone(), v[2].clone())
}

fn area2(tpl: (i32, i32, i32)) -> i32 {
    let (a, b, c)  = tpl;
    2 * a + 2 * b + (a * b * c)

}
//Area calc based on puzzle spec.
fn area(tpl: (i32, i32, i32)) -> i32 {

    let (l, w, h) = tpl;
    let wh = w * h;
    let lh = l * h;
    let wl = w * l;

    let area = 2 * wl + 2 * wh + 2 * lh;
    let extra = cmp::min(wl, cmp::min(wh, lh));

    area + extra
}
/*
Parses line of input. eg "23x4x12" gives (23, 4, 12)
*/
fn split(triple: String) -> (i32, i32, i32) {
    let vec = triple.split("x").collect::<Vec<&str>>();
    (vec[0].parse::<i32>().unwrap(), vec[1].parse::<i32>().unwrap(), vec[2].parse::<i32>().unwrap())
}
