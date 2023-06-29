fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of {} is {}", s1, len);


    let s2 = String::from("hello");

    change(&mut s2);

}

fn calculate_length(s: &String) -> usize {
    s.len()

    // s는 값을 참조하지만, 소유권을 가지고 있지는 않다.
    // 따라서 scope를 벗어나도 값을 버리지 않는다.
}

// 기본적으로 &를 통해 빌려온 값은 수정할 수 없다.
// 하지만 &mut 로 수정을 허용할 수 있다.
// 대신 가변 참조자 변수는 딱 하나만 존재할 수 있다는 제한이 생긴다.
// 또한 불변 참조자가 존재하는 동안엔 가변 참조자는 생성이 불가능하다.
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
