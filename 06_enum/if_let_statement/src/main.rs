// if let을 이용한 간결한 흐름 제어
//
fn main() {

    // 한가지 경우만 매칭시키고 나머지는 버리는 match 구문이다.
    // 상당히 복잡하다.
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // 이렇게 작성할 수 있다.
    // 대신 모든 요소를 검사해주는 것을 잃어버렸다.
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // else 문을 포함할 수도 있다.
    else {
        println!("not three");
    }
}
