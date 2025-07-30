use std::io;

fn main() {
    let mut input = String::new();

    // 1. 첫 줄 입력: n
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // 2. 두 번째 줄 입력: 배열
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    // 3. 세 번째 줄 입력: 찾는 값 v
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let v: i32 = input.trim().parse().unwrap();

    // 4. 개수 세기
    let count = arr.iter().filter(|&&x| x == v).count();
    println!("{}", count);
}
