// 테스트 조직화하기
// 테스트는 크게 두 분류로 나뉜다.
// 1. 단위 테스트( unit test )
//  한 번에 하나의 모듈만 분리해서 테스트한다.
//  private 인터페이스를 테스트한다.
// 2. 통합 테스트( intergration test )
//  라이브러리 외부에 존재한다.
//  public 인터페이스를 이용한다.
//  테스트마다 여러 개의 모듈을 실험한다.
//  외부 코드가 라이브러리를 이용하는 방식으로 테스트한다.



// #[cfg(test)] 어노테이션은 cargo test때만 컴파일된다.
// 컴파일 시간을 줄여주고, 결과물의 크기를 줄여준다.
// 통합테스트는 다른 디렉터리에 존재하기때문에 이 어노테이션이 필요없다.
// cgf는 환경 설정을 의미하며, 러스트에게 뒤따르는 아이템이 특정한
// 환경 값에서만 포함되어야함을 말해준다.
//
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 비공개 함수를 테스트할 수 있다.
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
