fn main() {
    // s1이 some_string을 소유한다.
    let s1 = gives_ownership();

    // s2 선언
    let s2 = String::from("hello");

    // s3는 s2의 값을 소유한다.
    let s3 = takes_and_gives_back(s2);


    // 소유권과 반환값을 튜플로 받기 -> 불편하다.
    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s1);

    println!("the len of {} is {}", s5, len);
}

// some_string을 호출한쪽으로 넘긴다.
fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

// 인자로 받은 String을 호출한 쪽으로 넘긴다.
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// s와 길이를 반환한다.
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
