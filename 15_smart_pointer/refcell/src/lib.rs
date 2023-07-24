// 내부 가변성에 대한 용례 : mock 객체
//
// test double은 테스트하는동안 또다른 타입을 대신해 사용되는 타입을 
// 위한 일반적인 프로그래밍 개념이다.
// 목 객체는 테스트 중에 어떤 일이 일어났는지 기록해 정확한 동작이
// 일어났음을 단언할 수 있도록하는 testdouble의 특정한 타입이다.
//
// 러스트에 해당 의미의 객체는 존재하지 않는다.
// 하지만 구조체로 동일한 목적을 수행하는 객체를 만들 수 있다.
//
// 최대값에 맞서 값을 추적하고 현재 값이 최대값에 얼마나 근접한지를
// 메시지로 전송하는 라이브러리를 만들 것이다.
// 한 명에 유저에게 허용되는 API 호출 수의 허용량을 추적하는데
// 사용할 수 있다.

// 메시지를 전송하는 메커니즘을 제공한다.
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker { 
            messenger,
            value: 0, 
            max,
        }
    }

    // 값이 최대값에 얼마나 근접하는지를 추적하고 경고해주는 라이브러리
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

// set_value 함수는 테스트에 사용할 수 있는 반환값이 없다.
// 목 객체를 만들어 테스트할 수 있다.

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}


