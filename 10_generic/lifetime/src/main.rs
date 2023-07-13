// 라이프타임을 이용한 참조자 유효화
// 모든 참조자는 라이프타임을 가진다.
//

use std::fmt::Display;

fn main() {
    /*
    // 라이프타임은 댕글링 참조자를 방지한다.

    let r;
    {
        let x = 5;
        r = &x;
    }   // x는 이 시점에서 죽기때문에 r에 바인딩될 수 없다.

    println!("r: {}", r);
    */

    // 함수에서 제네릭 라이프타임
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    /* 라이프타임 명시 문법
    * '로 시작하며 짧은 이름을 가진다.
    * 보통 'a 이다.
    * &i32          <- 참조자
    * &'a i32       <- 라이프타임을 가지는 참조자
    * &'a mut i32   <- 라이프타임을 가지는 가변 참조자
    */

    func1();

    // 정적 라이프타임
    // Static Lifetime
    // 'static 라이프타임은 프로그램과 수명을 함께한다.
    // 모든 스트링 리터널은 'static 라이프타임을 가진다.
    // 아래처럼 명시할 수 있다.
    let s: &'static str = "I have a static lifetime.";
}

// 이 함수는 참조자가 x를 참조할지, y를 참조할 지를 몰라서
// 오류가 난다
// 빌림 검사를 하기 위해서는 제네릭 라이프타임을 추가해줄 필요가 있다.
// 'a 를 추가해 모두 동일한 라이프타임을 가짐을 명시하였다.
// 최소한 라이프타임 'a 만큼 살아있는 스트링 슬라이스임을 알려준다.
fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 구조체 정의에서 라이프타임 명시하기
// 참조자를 가지는 구조체의 경우 라이프타임의 명시가 필요하다.
// 이 경우, 구조체와 생사를 함께한다는 뜻이다.                                 
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn func1() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}

// 라이프타임의 생략
// 원래는 라이프타임을 생략할 수 없었다.
// 하지만 너무 자주 쓰이는 패턴이라 결정론적 패턴이 되었고
// 컴파일러가 충분히 추론할 수 있게 되었다.
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 함수, 메소드 파라미터에 대한 라이프타임을 "입력 라이프타임"이라한다.
// 반환값은 "출력 라이프타임"이다.
// 1. 참조자의 각 파라미터는 고유한 라이프타임 파라미터를 가진다.
// 파라미터가 2개면 라이프타임은 2개가 된다.
// 
// 2. 1개의 라이프타임 파라미터만 있다면, 그 라이프타임이 모든 출력
// 라이프타임 파라미터들에 대입된다.
// 
// 3. 여러개의 입력 라이프타임 파라미터가 있는데, 그 중 하나가 
// &self, &mut self라면 self의 라이프타임이 모든 출력 라이프타임에 대입

fn longest_with_an_announcement<'a, T>
    (x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    // 여기서 표시되려면 Display 바운드를 해야한다.
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
