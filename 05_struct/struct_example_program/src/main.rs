// 사각형 넓이를 계산하는 프로그램

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30
    };
    
    // println! 매크로는 다양한 종류의 포맷을 출력 가능
    // 구조체 사용 시 어떤 포맷으로 출력해야하는지 모른다.
    // :? 를 {} 사이에 넣어주면 Debug 포맷을 사용하게 할 수 있다.
    // 구조체 위에 #[derive(Debug)] 를 넣어줘야 동작한다.
    // :#? 으로 사용하면 각 요소가 한줄에 하나씩 보이게된다.
    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
        );
}

// &를 붙이지 않으면 rect1은 함수 실행 후 버려진다.
// &을 붙여 빌려오도록하면 함수 실행 후에도 살아있다.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.length
}
