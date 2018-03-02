use file_utils::read_file;

pub fn part1_part2(){
    let s = read_file("data/day1.txt".to_string());

    part1(&s);
    part2(&s);
}

fn part1(s: &String){
    let r1 =  countparens(s.to_string());
    println!("Day 1 part 1 {}", r1);

}
fn part2(s: &String){
    let r2 =  basement(s.to_string());
    println!("Day 1 part 2 {}", r2);


}
/*
Inc or dec, from zero, depending on if left or right parens.
*/
fn countparens(str: String) -> i32 {
    let mut net_value = 0;
    for c in str.chars(){
        match c {
            '(' => net_value = net_value + 1,
            _   => net_value = net_value - 1,

        }
    }
    net_value
}
/*
Modify location based on parens keeping count of changes. When location is -1 return count.
*/
fn basement(str: String) -> i32 {

    let mut location  = 0;
    for (count, c) in str.chars().enumerate(){
        match c {
            '(' => location = location + 1,
            _   => location = location - 1,
        }
        if location == -1 {return  count as i32}

    }
    location
}
