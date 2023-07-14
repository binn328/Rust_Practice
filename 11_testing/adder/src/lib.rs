// 테스트는 코드가 기대한대로 동작하는지를 검증하는 러스트 함수이다.
// 3가지 동작을 수행한다.
// 1. 필요한 데이터, 상태를 설정
// 2. 테스트할 코드를 실행
// 3. 그 결과가 예상대로인지 단언하기(assert)
//
// 러스트는 test속성과 매크로 몇 개, should_panic 속성을 제공한다.

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// 이 이노테이션이 테스트 함수임을 나타낸다.
// 테스트 함수를 #[cfg(test)]를 통해 명시할 필요가 있다.
// cargo test를 하면 프로젝트 내의 모든 테스트를 실행한다.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        // assert_eq! 를 사용하면 ==을 사용할 수 있다.
        // 아래의 뜻은 result == 4 라는 의미이다.
        assert_eq!(result, 4);
        // assert_ne! 도 있는데 !=을 사용할 수 있다.
        // 되어서는 안되는 값을 알고 있다면 사용할 수 있다.
        // 사용하려면 PartiaEq, Debug 트레잇을 구현해야한다.
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        // 만약 larger가 smaller를 감쌀 수 있다면 panic!이 일어나지않는다.
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        // 이 테스트가 통과하려면 !를 통해 not을 해줘야한다.
        assert!(!smaller.can_hold(&larger));
    }

    // 에러메시지에 커스텀 메시지를 넣을 수 있다.
    // 어떤 값이 들어있는지 확인할 때 유용하다.
    // 그냥 두번째 인자로 포맷을 넣어주면 된다.
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
            "Greeting did not contain name, value was '{}'", result);
    }

    // should_panic으로 패닉 체크하기
    // 이 속성을 사용하면 패닉을 일으켜야 테스트가 통과된다!
    // expect 파라미터를 추가해 실패 메시지가 제공된 메시지에 포함되는지 
    // 확실하게 할 수 있다.
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

// assert! 매크로를 이용하기
// assert! 매크로에 true가 제공되면 아무것도 하지 않는다.
// 하지만 false가 제공되면 panic!을 호출하여 테스트를 실패하게한다.
// 코드가 의도한대로 동작하는지 체크하는 것을 도와주는 매크로이다.

#[derive(Debug)]
pub struct Rectangle {
    length: u32, 
    width: u32,
}

// can_hold 메소드는 boolean값을 반환한다.
// 이는 assert! 매크로를 쓰기 아주 좋은 사례이다.
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

// 커스텀 실패 메시지 추가하기
pub fn greeting(name: &str) -> String {
    format!("Hello")
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1  {
            panic!("Guess value must be greater than 1, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}");
        }

        Guess {
            value
        }
    }
}
