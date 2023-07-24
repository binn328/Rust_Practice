// Deref 트레잇을 가지고 스마트 포인터를 평범한 참조자처럼 취급하기
//
// Deref 트레잇을 구현하면 * 동작을 커스터마이징할 수 있다.
use std::ops::Deref;

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 비슷한 MyBox를 만들고 똑같이 해보자
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // MyBox는 역참조가 될 수 없다는 오류가 발생한다.
    assert_eq!(5, *y);
    
    example();
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
     fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
   
    fn deref(&self) -> &T {
        &self.0
    }
}

// 암묵적 역참조 강제
// Deref를 구현한 어떤 타입의 참조자를 Deref가 본래의 타입으로부터 
// 변형 가능한 타입의 참조자로 바꾸어준다.
// 함수에 매개변수를 넘기거나 할 때 자동적으로 발생한다
fn example() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // 만약 이 기능이 없었다면...
    hello(&(*m)[..]);
}
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// * 을 오버 라이딩하기 위한 DereMut 트레잇이 존재한다.
// 3가지 경우에 역참조를 강제한다.
// 1. T: Deref<Target=U> 일때 &T -> &U
// 2. T: DereMut<Target=U> 일때 T -> &mut U
// 3. Deref<Target=U> 일때 &mut T -> &U
//
