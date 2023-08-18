/*  고급 트레잇
    
    - 연관 타입은 트레잇 정의 내에서 플레이스홀더 타입을 명시합니다.
         우리는 트레잇이 구현되지 전까지 어떤 타입이 필요한지 정확히 알 필요없이
        임의의 타입을 사용하는 트레잇을 정의할 수 있다. 
         가장 대표적인 예는 Iterator 트레잇이다. Item이라는 이름의 연관타입이 있고
        Iterator 트레잇이 구현하는 타입이 반복하는 값의 타입을 대신한다.*/

// Item이 플레이스홀더 타입이고 next 메소드는 Option<Self::Item> 을 반환한다.
// 이를 구현하는 사람은 Item의 구체적인 타입을 명시할 것이다.
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self:Item>;
}

/* 제네릭을 써도 되는데 왜 이걸 쓸까? */
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {};
}

// 제네릭을 쓰면 각 구현마다 타입을 명시해야한다.
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

/* 기본 제네릭 타입 파라미터와 연산자 오버로딩 
 * 제네릭 타입을 사용할 때, 해당 제네릭 타입에 대한 기본 구체타입을 명시할 수 있다.
 * 기본 타입이 동작할 경우, 트레잇을 구현할 사람이 구체적인 타입을 명시하는 수고를
 * 덜어준다. 명시 문법은 제네릭 선언 시 <PlaceholderType=ConcreteType> 이다.
 * 
 * 이는 연산자 오버로딩과 함께 사용할 때 유용하다. 
 * 기본적으로 러스트는 연산자를 만들거나 오버로딩하는걸 허용하지는 않는다.
 * 하지만 std::ops에 나열된 연산자와 연관지어 구현하는 것으로 연산자와 트레잇을
 * 오버로딩할 수 이싸.*/

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3}, Point { x: 3, y: 3 });
}

trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}