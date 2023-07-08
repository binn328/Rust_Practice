// enum 정의하기
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// variant에는 여러 데이터타입이 들어갈 수 있다(enum도 가능!)
// 각자 다른 구조체를 구현한다고 생각하면 비슷하다.
// 하지만 그런식으로 구현하면 한번에 여러 구조체를 입력받는게 어렵다.
// struct의 impl처럼 enum에도 메소드를 구현할 수 있다.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //call 메소드
    }
}

// IP 주소 구조체로 구현하기
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // 열거형 인스턴스 생성
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // route 함수 사용하기
    route(four);
    route(IpAddrKind::V6);

    // struct를 이용한 IP 주소
    /*
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1");
    }; 
    */
    
    // enum에 (String)을 추가하여 간단하게 할 수 있다.
    //let home = IpAddrKind::V4(String::from("127.0.0.1"));

    //서로 다른 걸 넣을 수도 있다.
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    // 물론 표준 라이브러리에 IP enum은 이미 정의가 되어있다.


    // enum을 생성하고 call 메소드를 호출
    let m = Message::Write(String::from("hello"));
    m.call();
}

// enum을 매개로 받는 함수
fn route(ip_type: IpAddrKind){

}
