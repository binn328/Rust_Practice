// 미국의 주 열거형
#[derive(Debug)]
enum UsState {
    Albama,
    alaska,
    wacingtandc,
}
// 미국 동전 열거형
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// 미국 동전을 받아 센트 단위로 반환하는 함수
fn value_in_cents(coin: Coin) -> u32 {
    // match 뒤에 표현식을 써준다, 여기선 coin 이다.
    // if는 boolean만 가능하지만 여기서는 어떤 타입이든 가능하다.
    match coin {
        // 패턴 => 실행될 코드,
        // 중괄호를 통해 여러줄의 코드를 실행할 수도 있다.
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 이런식으로 작성하면 값이 state 에 바인딩되어 사용할 수 있게 된다!
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// Option<T> 를 이용한 매칭
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // 아무 값도 안들어있으면 None을 반환하고 아무 연산도 안함
        None => None,
        // 내부에 값이 있으면 1을 더해 반환
        Some(i) => Some(i + 1),
    }
}
// match 문은 모든 케이스를 다루지 않으면 에러를 일으킨다.

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    // _ 변경자(placeholder)
    // 1, 3, 5, 7 까지만 match로 다루고 싶다.
    // u8을 사용하면 0 ~ 255까지 다루어야함'
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // _ 는 이전에 명시하지 않은 모든 가능한 경우와 매칭된다.
        // default 구문과 비슷하다.
        _ => (),
    }

    // 한가지 경우만 다루고 싶을때 match를 사용하는건 너무 장황하다.
    // 때문에 if let이 제공된다.
}
