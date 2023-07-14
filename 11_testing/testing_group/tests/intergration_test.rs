// 외부 테스트를 위해서는 프로젝트 최상위에 tests 디렉터리를 만들어야한다.
// 원하는 만큼의 테스트파일을 만들 수 있다.
// cargo는 각 파일을 개별적인 크레이트처럼 컴파일한다.

// tests 내부의 각 테스트는 개별적인 크레이트이다.
// 때문에 라이브러리를 가져와야한다.
// 이 디렉터리의 파일들은 cargo test 시에만 컴파일 된다.
//
// 특정 테스트만 실행하고 싶다면
// cargo test --test [test파일이름.rs] 
// 를 해주면 된다.
extern crate testing_group;

// 서브 모듈을 가져올 수 있다!
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, testing_group::add_two(2));
}

// 만약 lib.rs가 없고 main.rs만 있는 바이너리 프로젝트하면
// tests 디렉터리에 통합 테스트 파일을 만들고 main.rs에 있는
// 함수를 가져오기 위해 extern crate를 사용할 수 없다.
// 오직 라이브러리 크레이트만 다른 크레이트에서 호출하고 사용할 수 있는
// 함수들을 노출시킨다.
// 바이너리 크레이트는 그 스스로 실행될 것으로 여겨지기때문이다.
//
