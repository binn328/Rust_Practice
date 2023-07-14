// 커맨드라인 옵션을 통해 테스트의 실행방식을 제어할 수 있다.
// 
// 예를 들어 각 테스트는 각 스레드에서 진행된다.
// 만약 싱글 스레드로 실행하고 싶다면 스레드 수를 1개로 제한하면 된다.
// cargo test -- --test-threads=1 
// 만약 여러 테스트가 간섭을 일으킬 여지가 있다면 이를 이용할 수 있다.
//

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;
   
    // 기본적으로 테스트 통과 시 println! 출력을 볼 수 없다.
    // 실패하면 볼 수 있다.
    // 성공시에도 보고싶다면 cargo test -- --nocapture 를 써주면 된다.
    // 근데 안되는듯함
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
    
    // #[ignore] 이노테이션으로 특정 함수는 테스트에서 배제할 수 있다.
    // 만약 무시된 테스트만 실행하고 싶다면
    // cargo test -- --ignored 로 가능하다.
    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    // 만약 특정 함수만 테스트하고 싶다면
    // cargo test this_test_will_fail 
    // 이런식으로 함수이름을 넘겨주면 된다.
    // 대신 하나만 가능하며, 여러개는 불가능하다.
    //
    // 하지만 함수이름의 일부분을 특정하면 여러개를 실행할 수 있다.
    // cargo test this_test_will_
    // 이러면 해당 이름이 포함된 함수가 모두 실행된다.
}
