fn main() {
    let mut s = String::from("hello world");

    let word = first_word_n(&s[..]);

    let s_literal = "hello world";

    let word = first_word_n(&s_literal[..]);

    let word = first_word_n(s_literal);
    
    println!("the first word is {}", word);


    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
}

fn first_word(s: &String) -> usize {
    // 공백을 확인하기 위해 byte 배열로 변환한다.
    let bytes = s.as_bytes();

    // 공백문자를 나타내는 바이트 리터널을 찾는다 
    // 찾았다면 그 인덱스를 반환한다.
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    // 아니면 그냥 길이를 반환한다.
    s.len()
}

// 슬라이스를 적용한 개선판
fn first_word_n(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) {
    let s = String::from("hello world");

    // 슬라이스이다. 
    // 문자열의 일부를 참조하는 String Slice 이다.
    let hello = &s[0..5];
    let world = &s[6..11];
}
