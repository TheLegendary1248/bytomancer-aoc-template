use super::solutions::final_answer;
use super::solutions::input_raw;

const DAY: u8 = 01;
const SOL: u8 = 1;

pub async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    lines
}

pub async fn solve(submit: bool, example: bool) {
    let input = input(example).await;
    let mut pos = 50;
    let mut zeroCount = 0;
    for element in &input {
       let num: i32 = &element[1..].parse().unwrap() * if &element.chars().next() == &Some('L') { -1 } else { 1 };
       pos += num;
       if pos % 100 == 0 { zeroCount+=1; }
    }
    final_answer(zeroCount.to_owned(), submit, DAY, SOL).await;
}
