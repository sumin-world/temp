use std::io;

fn main() {
    // 1. 사용자 입력을 저장할 String 변수 생성
    let mut input = String::new();

    // 2. 표준 입력(키보드)에서 한 줄을 읽어 input에 저장
    // .read_line()은 Result 타입을 반환하므로 unwrap()으로 에러 처리 (성공 가정)
    io::stdin().read_line(&mut input).unwrap();

    // 3. 입력된 문자열을 정수(usize)로 전환
    // .trim()으로 공백(개행 문자 포함) 제거
    // .parse()로 문자열을 정수로 변환, .unwrap()으로 변환 실패 시 패닉 처리 (성공 가정)
    let n: usize = input.trim().parse().unwrap();
    
    // 4. 1부터 n까지 반복하여 출력
    for i in 1..=n {
        println!("Hello World, Judge {}!", i);
    }

}