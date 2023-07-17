extern crate greprs;

// 인자값을 받기 위해 사용
// 반복자를 이용해 인자들을 프로그램에 전달한다.
// 반복자는 다음 특성을 가진다.
// 1. 반복자는 하나의 연속된 값을 생성한다.
// 2. 반복자에 collect 함수를 사용해 일련의 값을 벡터로 변환할 수 있다.
use std::env;
use std::process;

use greprs::Config;

fn main() {
    // 오류가 없으면 Config를, 오류나면 프로그램을 종료시킨다.
    // 익명 함수를 클로저라고 부른다.
    // |err| 파이프 사이의 인자를 클로저로 전달한다.
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // eprintln! 은 표준 에러에 출력한다.
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    // if를 통해 run이 Err값을 반환하는지 검사한다.
    // 만약 그렇다면 종료한다.
    // unwrap_or_else는 사용할 필요가 없다.
    // 성공 시 ()를 반환하기 때문이다.
    // 에러만 신경쓰면 된다.
    if let Err(e) = greprs::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}


