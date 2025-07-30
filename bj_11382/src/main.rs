use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<i64> = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    
        let a = numbers[0];
        let b = numbers[1];
        let c = numbers[2];

        let sum = a + b + c;
        println!("{}", sum);
}