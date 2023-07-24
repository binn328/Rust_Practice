use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {

    // 말단 노드
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // 말단 노드를 자식으로 가지는 branch 노드
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // 자식은 부모의 존재를 모른다.
    // 부모를 알게 하려면 Node 구조체에 부모를 가리키는 걸 추가해야한다.
    // 단순하게 Rc<T>를 넣으면 순환참조를 발생시킬 것이다.
    // Weak<T>를 사용해야한다.
    // 부모를 참조는 하지만, 소유권은 가지지 않는다.

}
