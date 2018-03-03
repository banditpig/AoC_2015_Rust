//It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
//It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
//It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
use utils::read_file;

static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
static PAIRS: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub fn part1_part2() {
    let s = read_file("data/day5.txt".to_string());
    part1(&s);
    part2();
}

fn is_acceptable(s: &str) -> bool {
    doesnt_contain_substrs(s) && has_three_vowels(s) && has_pair(s)
}

fn has_three_vowels(s: &str) -> bool {
    let mut cnt = 0;
    for c in s.chars() {
        if VOWELS.contains(&c) { cnt += 1 }
        if cnt >= 3 { return true; }
    }
    false
}

fn doesnt_contain_substrs(s: &str) -> bool {
    for pair in PAIRS.into_iter() {
        if s.contains(pair) { return false; }
    }
    true
}

fn has_pair(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut ix = 0;
    loop {
        if ix < (bytes.len() - 1) && bytes[ix] == bytes[ix + 1] { return true; }
        ix += 1;
        if ix == bytes.len() { return false; }
    }
}

fn part1(s: &str) {
    let c = s.lines().filter(|&x| is_acceptable(x)).count();
    println!("Day 5 part 1 {} ", c);
}

fn part2() {
    println!("Day 5 part 2");
}