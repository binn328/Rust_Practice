pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

// use 키워드를 이용하면 긴 함수 호출을 줄여줄 수 있다.
// 특정 함수만 가져오는 것도 가능하다.
use a::series::of::nested_modules;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// enum 에서 일부만 가져와 사용하는 것도 가능하다.
use TrafficLight::{Red, Yellow};
// user TrafficLight::*;
// * 구문을 사용하여 모두 가져오는 것 또한 가능하다.
// * 은 glob 이라 불린다.
// 너무 자주쓰면 이름 충돌 문제를 일으킬 수 있으므로 남발해서는 안된다.
fn main() {
    // 절대경로를 지정하는 것은 너무 길어진다.
    a::series::of::nested_modules();

    // 이렇게 
    nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
