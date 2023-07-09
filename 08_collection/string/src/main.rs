fn main() {
    // 빈 String 생성
    let mut s = String::new();

    // to_string을 통해 
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    // String::from 을 사용할 수도 있다.
    let s = String::from("initial contents");

    // String은 UTF-8 로 인코딩되기때문에 UTF-8의 모든 요소를 담을 수 있다.
    
    // String 추가하기
    let mut s = String::from("foo");
    s.push_str("bar");
    // s = foobar

    // push_str() 은 소유권을 가져오지 않는다.
    // 따라서 아래의 함수는 잘 동작한다.
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s2 is {}", s2);

    // push는 하나의 문자를 받아 추가한다.
    s1.push('l');


    // + 연산자를 사용할 때는 s1 + &s2 형태로 사용된다.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // 이 동작 이후 s1은 유효하지 않다.
    let s3 = s1 + &s2;

    // 여러 String을 합칠 땐 다루기가 불편해진다.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // format! 매크로를 사용하면 편해진다.
    // 또한 소유권도 가져가지 않는다.
    // 읽기도 편하다!
    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3);
    

    // String의 인덱싱
    let s1 = String::from("hello");
    // Rust String은 인덱싱을 지원하지 않는다.
    // let h = s1[0];
    
    // String은 Vec<u8> 을 감싼 것이다.
    // 이 경우 Hola를 저장하는 Vec이 4Byte 길이라는 뜻이다.
    // UTF-8로 인코딩되면 각 글자는 1Byte라는 뜻이다.
    let len = String::from("Hola").len();

    // 이건 어떨까
    let len = String::from("김치").len();
    println!("{}", len);
    // 얘는 6이 나온다.
    // 유니코드를 사용하기에 항상 같은 값을 가지지는 않는다.
    // 때문에 인덱싱을 사용할 수 없는 것이다.

    // 러스트 관점에서 문자열을 바라보는 방식은 3가지가 있다.
    // 1. u8로 바라보기
    // 2. char 로 바라보기
    // 3. 문자소 클러스터로 바라보기

    // String 슬라이싱 하기
    // 위와 같은 이유로 슬라이싱은 좀 복잡해진다.
    let hello = "감자떡바나나";
    // 이 동작은 3개씩 끊지 않으면 런타임 오류를 발생시킨다.
    let s = &hello[0..3];

    // 개별적인 유티코드 스칼라 값에대한 연산을 원한다면
    // chars 메소드를 사용할 수 있다.
    for c in "감자떡바나나".chars() {
        println!("{}", c);
    }

    // bytes는 가공되지 않은 바이트를 반환한다.
    for b in "감자떡바나나".bytes() {
        println!("{}", b);
    }

    // 하나의 유니코드 스칼라값은 여러개의 바이트값일 수 있다.



    
}
