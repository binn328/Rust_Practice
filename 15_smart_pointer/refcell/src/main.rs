// RefCell<T>와 내부 가변성 패턴
// 
// 불변 참조자가 있더라도 데이터를 변형할 수 있도록 해준다.
// 원래 빌림 규칙에 의해 허용되지 않지만, 
// unsafe 코드를 사용해 가능하게 만들었다.

// RefCell<T>는 데이터 상에 단일 소유권을 나타낸다.
//
// Box<T>의 빌림 규칙 불변성은 컴파일 타임에 집행된다.
// 잘못된 동작 시 컴파일 시에 실패할 것이다.
//
// RefCell<T>는 불변성이 런타임에 집행된다.
// 잘못된 동작 시 실행 도중에 panic!을 일으키고 종료될 것이다.
//
// RefCell<T>도 싱글 스레드에서만 사용이 가능하다.
//
// Rc<T> : 동일한 데이터에 대해 여러 개의 소유자를 허용한다.
// Box<T> : 컴파일 타임에 검사된 불변, 사변 빌림을 허용한다.
// RefCell<T> : 런타임에 검사된 가변 빌림을 허용한다.
//              때문에 RefCell<T>가 불변일 때도 내부 값을 변경할 수 있다.

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // 빌림 규칙으로 인해 불변값을 가변하게 빌릴 수 없다.
    // let x = 5;
    // let y = &mut x;
    

    
    // Rc<T>와 RefCell<T>를 조합하여 가변 데이터의 복수 소유자 만들기

    // RefCell<T>를 사용하는 일반적인 방법은 Rc<T>와 조합하는 것이다.
    // 변경 가능하면서 복수의 소유자를 가지는 값을 만들 수 있다!
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
