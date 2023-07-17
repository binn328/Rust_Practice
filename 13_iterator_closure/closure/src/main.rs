// 클로저는 변수를 저장하거나 다른 함수에 인자로 넘길 수 있는 익명함수다.
// 한 곳에서 클로저를 만들면 다른 문맥에서 호출해 평가할 수 있다.
// 함수와 달리 클로저는 호출되는 스코프로부터 변수를 캡쳐할 수 있다.

use std::thread;
use std::time::Duration;

fn main() {
    // 대충 입력과 랜덤값 생성을 가장한 하드코딩된 코드
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

// 클로저로 행위를 추상화하기
// 2초동안의 계산 후 값을 돌려준다.
//
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// 이 코드는 2초짜리 코드를 여러번 호출한다.
// 함수를 이용해 리팩토링 할 수 있다.
// 해당 함수의 호출을 한 곳으로 추출하고 결과를 변수에 저장했다.
// 아니면 그냥 변수에 클로저 자체를 집어넣을 수도 있다!

fn generate_workout(intensity: u32, random_number: u32) {
    // 파이프로 클로저에 대한 파라미터를 표시한다.
    // 두 개를 가지면 |num1, num2| 콤마로 구분한다.
    // 반환값은 세미콜론이 없는 표현식이 된다.
    // 이 구문은 익명함수의 정의를 포함한다.
    // 하지만 결과값은 포함하지 않는다.

    // 또한 클로저는 파라미터나 반환값의 타입을 명시하지 않아도 된다.
    // 타입 어노테이션을 정의하면 타입을 깐깐하게 확인한다.
    // 클로저는 제한된 문맥에서만 사용되고, 때문에 추론이 가능하다.
    // 굳이 깐깐하게 하고싶다면 타입 어노테이션을 추가할 수 있다.
    // 이렇게 하면 fn과 비슷하게 보인다.

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity) 
            );
        }
    }
}

// 클로저와 클로저를 호출한 결과값을 가지는 구조체를 만들 수 있다.
// 이 방식은 구조체가 결과값이 필요할 때만 클로저를 호출한다.
// 결과값은 캐시에 저장해두어 필요할 때 꺼내사용할 수도 있다.
// 이를 메모이제이션(지연평가) 방식이라고 한다.
//
// 구조체가 클로저를 가지기 위해서는 클로저타입을 알려주어야한다.
// 각 클로저 인스턴스는 자신의 유일한 익명 타입을 가진다.
// 동일한 타입을 가져도 다른 것으로 간주된다는 뜻이다.
// 이를 위해 제네릭과 트레잇 바운드를 사용한다.
//
// Fn 트레잇은 표준 라이브러리에서 지원한다.
// 모든 클로저들은 Fn, FnMut, FnOnce 중 하나의 트레잇을 구현한다.

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T> 
    where T: Fn(u32) -> u32 
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

// 이미 1이 저장되어있으면 이후에도 1이 나온다는 단점이 존재한다.
// 해시맵을 사용하면 해결할 수 있다.
// u32 -> u32 만 가능하다는 단점도 있다.
// 이를 해결하려면 제네릭을 활용할 수 있다.
#[test]
fn call_with_diffrent_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
