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
