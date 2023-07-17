// 반복자로 일련의 항목들 처리하기
//
// 반복자 패턴은 일련의 항목들에 대해 순서대로 어떤 작업을 수행하도록
// 도와주는 역할을 한다.
// 각 항목들을 순회하고, 종료될 시기를 정하는 로직을 담당한다.

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    // 반복자를 생성하는 다른 메소드들
    // map이 대표적이다.
    // 새 반복자를 생성하기 위해 각 항목에 대해 호출할 클로저를 받는다.
    // 여기서 클로저는 벡터의 각 항목에서 1이 증가된 새로운 반복자를 만든다.
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

// 반복자는 Iterator 라는 트레잇을 구현한다.
/*
* trait Iterator {
*   type Item;
*   fn next(&mut self) -> Option<Self::Item>;
* }
*/
// next 메서드만 구현하면 된다.

// next를 호출해 반복자의 다음값을 불변 참조로 가져올 수 있다.
// 소유권을 가지고 싶다면 into_iter() 를 호출해야한다.
// 가변 참조를 하고 싶다면 iter_mut() 를 호출해야한다.
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    
    assert_eq!(v1_iter.next(), Some(&1));
    
    assert_eq!(v1_iter.next(), Some(&2));

    assert_eq!(v1_iter.next(), Some(&3));

    assert_eq!(v1_iter.next(), None);

}

// 반복자를 소비하는 메소드들이 있다.
// 이 친구들은 반복자를 넣으면 써버린다.
// sum() 이 그 예시이다.
// 반복자의 소유권을 가져와 next()를 계속 호출해 완료하면 결과를 반환한다.
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // sum() 호출 이후로 v1_iter는 사용할 수 없다.
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
