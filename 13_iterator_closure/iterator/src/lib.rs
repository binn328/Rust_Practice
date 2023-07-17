#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// filter 반복자 어댑터를 사용해 환경을 캡쳐하는
// 클로저의 일반적인 사용법
// filter 메소드는 반복자로부터 각 항목을 받아 Boolean을 반환하는
// 클로저를 인자로 받는다. true를 반환한다면 반복자에 포함된다.
// false를 반환한다면 반복자에 포함되지 않는다.
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)    // 맞는 사이즈만 반복자에 포함
        .collect() 
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
