use crypto::md5::Md5;
use crypto::digest::Digest;


pub fn part1_part2() {
    part1();
    part2();
}

fn part1() {
    let p1 = run_for_n_zeros(5);
    println!("Day 4 part 1 {}", p1);
}

fn part2() {
    let p2 = run_for_n_zeros(6);
    println!("Day 4 part 2 {}", p2);
}

fn run_for_n_zeros(n: usize) -> i32 {
    let mut hasher = Md5::new();
    let key = "yzbqklnj".as_bytes();
    let mut i = 0;
    loop {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let hash = hasher.result_str();

        if starts_with_n_zeroes(n, &hash) {
            return i;
        }

        i += 1;
        hasher.reset();
    }
}

fn starts_with_n_zeroes(n: usize, str: &String) -> bool {
    let first_n: String = str.chars().skip(0).take(n).collect();
    first_n == "0".to_string().repeat(n)
}