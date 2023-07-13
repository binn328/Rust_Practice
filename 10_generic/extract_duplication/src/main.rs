// 함수를 추출해 중복을 없애기

// 가장 큰 수를 찾는 코드
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let mut biggest = numbers[0];

    for number in numbers {
        if number > biggest {
            biggest = number;
        }
    }

    println!("The largest number is {}", biggest);

    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);

    println!("The largest number is {}", result);

    // 한 번 더 하기
    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut biggest = numbers[0];

    for number in numbers {
        if number > biggest {
            biggest = number;
        }
    }

    println!("The largest number is {}", biggest);

    // 같은 코드를 두 번 쓰는건 매우 귀찮다.
}

// 함수를 사용하면 코드 중복을 없앨 수 있다.
// list는 i32 슬라이스들을 나타낸다.
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
