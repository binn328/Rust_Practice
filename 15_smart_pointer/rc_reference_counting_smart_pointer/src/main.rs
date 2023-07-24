// Rc<T> 참조 카운팅 스마트 포인터
//
// 보통 소유권은 명확하다.
// 하지만 하나의 값이 여러개의 소유자를 가질 수 있는 경우도 있따.
// 그래프 데이터 구조에서 여러 엣지가 동일한 노드를 가리킬 수 있다.
// 그 노드는 해당 노드를 가리키는 모든 엣지들에 의해 소유된다.
// 노드는 어떤 엣지가 가리키고있는동안에는 정리되어서는 안된다.
// 
// 여러 부분에서 읽을 데이터를 힙에 할당하고, 어디서 데이터를 마지막으로
// 활용할지를 컴파일 타임에 알 수 없는 경우에는 Rc<T> 타입을 사용한다.
//
// Rc<T>는 단일 스레드에서만 사용할 수 있다.

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // Rc::clone을 호출해 a의 Rc<List>에 대한 참조자를 넘긴다.
    // a.clone을 쓸 수도 있지만 관례상 Rc::clone을 사용한다.
    // a.clone은 깊은 복사를 하는데 시간이 오래 걸린다.
    // Rc::clone은 단순히 카운트만 증가시킨다.
    // 또한 시각적으로 두 개를 구분할 수 있어 디버깅에 도움이 된다.
    // 성능상의 문제가 발생했다면 a.clone만 건드리면 되는 것이다.
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// Rc<T>는 읽기 전용으로 여러 부분에 데이터를 공유하도록 허용해준다.
// 가변으로 공유하게 된다면 빌림 규칙을 위반할지도 모른다.
// RefCell로 가변을 사용할 수 있다.
