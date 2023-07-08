// Option 열거형
// Option 타입은 값이 있거나 없을 수도 있는 상황을 나타낼 수 있다.
// 러스트에는 null 타입이 없다.
// Option<T>는 Some(T), None 을 가진다.

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    // None을 사용한다면 추론이 안되므로 타입을 명시해줄 필요가 있다.
    let absent_number: Option<i32> = None;

    // Some<T> 는 T와는 다른 타입이다.
    
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // 그래서 이 코드는 실행이 되지 않는다.
    //let sum = x + y;

    // 사용하기 전에 T로 변환이 필요하다.
    // null을 가질 수 있는 값이라면 Option enum을 사용해주어야한다.
    // 그래야 명시적으로 null 오류를 막을 수 있기 때문이다.
    // Some(T) 인경우를 처리하는 코드를 만들어 T를 사용할 수 있다.
    // None인 경우에 실행될 코드도 필요하다.
    // 이를 위해서 Match 구문을 사용할 수 있다.

}
