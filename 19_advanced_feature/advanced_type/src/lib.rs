fn _type_new() {
    // Kilometers와 i32는 동의어
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    // 더할 수 있지만, 타입 검사의 이점은 없다.
    println!("x + y = {}", x + y);

    // type을 통해 긴 타입을 줄일 수 있다.
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    // Result<..., Error>가 많이 반복될 때도 사용할 수 있다.
    // type Result<usize> = Result<usize, std::io::Error>;
}

/* 반환하지 않는 ! 타입 */
// ! 타입은 부정 타입이라고 한다.
// 빈 타입이다.
// 반환하지 않는 함수는 발산 함수라 부른다.
fn bar() -> ! {}
