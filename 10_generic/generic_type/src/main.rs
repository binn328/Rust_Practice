// 제네릭 데이터 타입
// largest 함수를 i32, char 형식에 동시에 적용할 수는 없을까?
// 제네릭을 사용하면 가능하다!
//
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);

    // 제네릭 구조체
    let integer = Point { x:5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let p1 = Duel_Point { x: 5, y: 10.4 };
    let p2 = Duel_Point { x: "Hello", y: 'c' };
    
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// 타입 T를 선언, list는 T 타입값의 슬라이스를 가짐
// 아직 구현이 부족해 모든 T에 대해 동작하지않을 것이라고 오류가 난다.
// std::cmp::PartiaOrd 트레잇을 구현해야한다.
// 트레잇 바운드를 이용해 동작할 수 있는 타입만 받아들일 수 있다.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 제네릭 구조체
struct Point<T> {
    x: T, 
    y: T,
}

// 메소드 정의
// point.x() 로 x값을 얻을 수 있다.
// impl 뒤에 <T>를 정의해줘야한다.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 두 타입을 가질 수 있는 제네릭 구조체
struct Duel_Point<T, U> {
    x: T,
    y: U,
}

// 구조체 정의에서와 다른 제네릭 타입을 사용할 수 있다.
// x는 T타입, y는 W타입을 가지게 될 것이다.
impl<T, U> Duel_Point<T, U> {
    // 이 제네릭 타입은 해당 메소드에서만 작동한다.
    fn mixup<V, W>(self, other: Duel_Point<V, W>) -> Duel_Point<T, W> {
        Duel_Point{
            x: self.x,
            y: other.y,
        }
    }
}

// 제네릭 열거형
// Option이나 Result는 제네릭을 가지는 열거형이다.


// 제네릭 타입을 사용해도 전혀 느려지지 않는다!
// 러스트는 컴파일 타임에 제네릭 타입에 단형성화를 수행한다.
// 제네릭 코드를 실제로 채워질 구체적인 타입으로 된 코드로 바꾸는 과정이다.
// integer = Some(5); 를 사용했다면 컴파일 때
// enum Option_i32 {} 코드가 구체화되는 것이다.
