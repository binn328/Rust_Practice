struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {      
        self.length > other.length && self.width > other.width
    }

    // self 파라미터를 가지지 않아도 정의가 가능
    // 이 경우엔 Rectangle::square(3) 식의 연관함수로 사용한다.
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}
fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
}
