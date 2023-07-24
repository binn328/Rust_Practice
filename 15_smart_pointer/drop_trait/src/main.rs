// Drop 트레잇은 값이 스코프를 벗어났을 때 수행할 동작을 커스터마이징한다.
// 네트워크 자원을 해제한다던가 하는 일이 가능하다.
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");

    example01();
}

// std::mem::drop 을 통해 값을 일찍 버릴 수도 있다.
// drop 을 수동으로 실행하는 것은 불가능하다.
// 대신 std::mem::drop 메소드를 호출해야만한다.

fn example01() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    // 이 동작은 허용되지 않는다.
    // c.drop();
    // 이렇게 해야한다.
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

}
