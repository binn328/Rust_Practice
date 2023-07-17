// Iterator 트레잇으로 나만의 반복자 만들기
//
// 자신만의 타입에 트레잇을 구현하면 원하는 동작을 하는 반복자를 만들 수 있다.
// 1 에서 5까지 셀 수 있는 반복자를 만들어보자

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    // count값이 6보다 작다면 count를, 아니면 None을 반환한다.
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

}

// 다른 Iterator 메서드를 이용하기
#[test]
fn using_ohter_iterator_trait_methods() {
    // Count 인스턴스로 인해 생성된 값을 얻고
    // 다른 Counter 인스턴스로 생성된 값과 쌍을 이루고
    // 각 쌍을 곱하고
    // 3으로 나눠지는 값들만 유지하며
    // 모든 결과값을 더한다.
    //
    // zip 은 4개의 쌍을 생성한다.
    // None은 생성되지 않는다.
    // 1, 2, 3, 4 까지만 생성된다.
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}
